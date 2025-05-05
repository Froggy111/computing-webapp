use crate::define_init_task;
use crate::define_task;
use crate::libs::types::errors::ErrorStr;
use crate::libs::types::shared::*;

use leptos::html::Canvas;
use leptos::logging;
use leptos::prelude::*;
use leptos_use::{use_raf_fn, use_resize_observer, UseRafFnCallbackArgs};

use wasm_bindgen::JsCast;
use web_sys::{ResizeObserverEntry, WebGl2RenderingContext};

use std::cell::{Ref, RefMut};
use std::fmt;
use std::rc::Rc;

pub trait InitTaskFnTrait = FnMut(&WebGlCanvas) -> bool + 'static;
define_init_task!(InitTask, InitTaskFn, InitTaskFnTrait, (canvas: &WebGlCanvas), bool);

pub trait ResizeTaskFnTrait =
    FnMut(&WebGlCanvas, &ResizeObserverEntry) -> Result<(), ErrorStr> + 'static;
define_task!(ResizeTask, ResizeTaskFn, ResizeTaskFnTrait, (canvas: &WebGlCanvas, resize: &ResizeObserverEntry), Result<(), ErrorStr>);

pub trait RafTaskFnTrait = FnMut(&WebGlCanvas, RafTime) -> Result<(), ErrorStr> + 'static;
define_task!(RafTask, RafTaskFn, RafTaskFnTrait, (canvas: &WebGlCanvas, timestamp: RafTime), Result<(), ErrorStr>);

pub struct RafTime {
    delta: f64,
    timestamp: f64,
}

impl RafTime {
    fn new(delta: f64, timestamp: f64) -> Self {
        Self { delta, timestamp }
    }
}

#[derive(Debug, Clone)]
pub struct WebGlCanvas<'a> {
    // data
    context: SharedRefCell<Option<WebGl2RenderingContext>>,
    canvas_ref: SharedRefCell<Option<NodeRef<Canvas>>>,
    width: ReadSignal<u32>,
    set_width: WriteSignal<u32>,
    height: ReadSignal<u32>,
    set_height: WriteSignal<u32>,

    // synchronisation
    initialised: ReadSignal<bool>,
    set_initialised: WriteSignal<bool>,

    // tracking
    name: Rc<&'a str>,

    // tasks
    init_tasks: SharedRefCell<Vec<InitTask<'a>>>,
    resize_tasks: SharedRefCell<Vec<ResizeTask<'a>>>,
    // passes the time from last frame
    raf_tasks: SharedRefCell<Vec<RafTask<'a>>>,
}

impl<'a> fmt::Display for WebGlCanvas<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'a> WebGlCanvas<'a> {
    pub fn new(name: &'a str) -> Self {
        let init_context: Option<WebGl2RenderingContext> = None;
        let init_canvas_ref: Option<NodeRef<Canvas>> = None;
        let (init_width, init_set_width) = signal(0u32);
        let (init_height, init_set_height) = signal(0u32);
        let (init_initialised, init_set_initialised) = signal(false);
        Self {
            // data
            context: shared_ref_cell(init_context),
            canvas_ref: shared_ref_cell(init_canvas_ref),
            width: init_width,
            set_width: init_set_width,
            height: init_height,
            set_height: init_set_height,

            // synchronisation
            initialised: init_initialised,
            set_initialised: init_set_initialised,

            // tracking
            name: Rc::new(name),

            // tasks
            // all tasks should already be given a copy of WebGlCanvas in a closure, to reduce copy
            // operations
            init_tasks: shared_ref_cell(Vec::<InitTask>::new()),
            resize_tasks: shared_ref_cell(Vec::<ResizeTask>::new()),
            raf_tasks: shared_ref_cell(Vec::<RafTask>::new()),
        }
    }

    pub fn add_init_task(&self, task: InitTask<'a>) {
        self.init_tasks.borrow_mut().push(task);
    }

    pub fn add_resize_task(&self, task: ResizeTask<'a>) {
        self.resize_tasks.borrow_mut().push(task);
    }

    pub fn add_raf_task(&self, task: RafTask<'a>) {
        self.raf_tasks.borrow_mut().push(task);
    }

    pub fn get_context(&self) -> Ref<'_, Option<WebGl2RenderingContext>> {
        self.context.borrow()
    }

    pub fn get_context_mut(&self) -> RefMut<'_, Option<WebGl2RenderingContext>> {
        self.context.borrow_mut()
    }

    pub fn get_canvas(&self) -> Ref<'_, Option<NodeRef<Canvas>>> {
        self.canvas_ref.borrow()
    }

    pub fn get_canvas_mut(&self) -> RefMut<'_, Option<NodeRef<Canvas>>> {
        self.canvas_ref.borrow_mut()
    }

    // this must be called inside the canvas component's setup code!
    // should never be called outside of a WebGlCanvasComponent (or derivative of that)
    pub fn setup(&self) {
        logging::log!("Setting up WebGlCanvas: {}", self);
        let canvas_ref = NodeRef::<Canvas>::new();
        self.set_canvas_ref(Some(canvas_ref));
    }

    // this must be called inside the canvas component's effect!
    // should never be called outside of a WebGlCanvasComponent (or derivative of that)'s effect
    pub fn init(&self) {
        if !self.initialised.get() {
            self.set_initialised.set(true);
            logging::log!("Initialising WebGlCanvas: {}", self);
            let canvas_ref_opt = (*self.canvas_ref.borrow());
            let canvas_ref = if let Some(canvas_ref) = canvas_ref_opt {
                canvas_ref
            } else {
                logging::error!("Canvas NodeRef is None, in {:?}", self);
                self.set_initialised.set(false);
                return;
            };
            let canvas = if let Some(canvas) = canvas_ref.get() {
                canvas
            } else {
                logging::error!("HtmlCanvasElement is None, in {:?}", self);
                self.set_initialised.set(false);
                return;
            };
            self.set_width.set(canvas.width());
            self.set_height.set(canvas.height());
            let canvas_element = &canvas;
            let context_opt = match canvas_element.get_context("webgl2") {
                Ok(context_opt) => context_opt,
                Err(error) => {
                    logging::error!(
                        "Error when getting canvas webgl2 context. Error: {:?}, in {:?}",
                        error,
                        self
                    );
                    self.set_initialised.set(false);
                    return;
                }
            };
            let context_obj = if let Some(context) = context_opt {
                context
            } else {
                logging::error!("Canvas webgl2 context is None, in {:?}", self);
                self.set_initialised.set(false);
                return;
            };
            let context = match context_obj.dyn_into::<WebGl2RenderingContext>() {
                Ok(context) => context,
                Err(_) => {
                    logging::error!(
                                        "Could not dyn_into canvas webgl2 context into webgl2 rendering context, in {:?}", self
                                    );
                    self.set_context(None);
                    self.set_initialised.set(false);
                    return;
                }
            };
            self.set_context(Some(context));
        }
        let mut tasks = self.init_tasks.borrow_mut();
        for task in tasks.iter_mut() {
            if !task.execute_if_uninitialised(self) {
                self.set_initialised.set(false);
            }
        }
    }

    pub fn run_resize_tasks(&self, entries: Vec<ResizeObserverEntry>) {
        if !self.initialised.get() {
            logging::error!("Called run_resize_tasks when uninitialised, in {:?}", self);
            return;
        }
        let mut tasks = self.resize_tasks.borrow_mut();
        for task in tasks.iter_mut() {
            let result = task.execute(self, &entries[0]);
            match result {
                Ok(_) => {
                    logging::log!("Resize task \"{}\" successfully executed in {}", task, self);
                }
                Err(error) => {
                    logging::error!(
                        "Resize task error: {}, for task {} in {:?}",
                        error,
                        task,
                        self
                    );
                }
            }
        }
    }

    pub fn run_raf_tasks(&self, timestamp: UseRafFnCallbackArgs) {
        if !self.initialised.get() {
            // this is log, as it is intended, and used for synchronisation
            // logging::log!("Called run_raf_tasks when uninitialised, in {:?}", self);
            return;
        }
        let mut tasks = self.raf_tasks.borrow_mut();
        for task in tasks.iter_mut() {
            let result = task.execute(self, RafTime::new(timestamp.delta, timestamp.timestamp));
            match result {
                Ok(_) => {
                    logging::log!("RAF task \"{}\" successfully executed in {}", task, self);
                }
                Err(error) => {
                    logging::error!("RAF task error: {}, for task {} in {:?}", error, task, self);
                }
            }
        }
    }

    fn set_canvas_ref(&self, canvas_ref: Option<NodeRef<Canvas>>) {
        let mut guard = self.canvas_ref.borrow_mut();
        *guard = canvas_ref;
    }

    fn set_context(&self, context: Option<WebGl2RenderingContext>) {
        let mut guard = self.context.borrow_mut();
        *guard = context;
    }
}

#[component]
pub fn WebGlCanvasComponent(web_gl_canvas: WebGlCanvas<'static>) -> impl IntoView {
    web_gl_canvas.setup();
    let canvas_ref_opt = *web_gl_canvas.canvas_ref.borrow();
    if let Some(canvas_ref) = canvas_ref_opt {
        let web_gl_canvas_effect = web_gl_canvas.clone();
        let web_gl_canvas_resize = web_gl_canvas.clone();
        let web_gl_canvas_raf = web_gl_canvas.clone();

        // effect to initialise WebGl in the canvas
        Effect::new(move |_| web_gl_canvas_effect.init());
        use_resize_observer(canvas_ref, move |entries, _| {
            web_gl_canvas_resize.run_resize_tasks(entries);
        });
        use_raf_fn(move |timestamp| web_gl_canvas_raf.run_raf_tasks(timestamp));

        view! { <canvas node_ref=canvas_ref>"Your browser does not support the canvas element."</canvas> }.into_any()
    } else {
        logging::error!("web_gl_canvas.canvas_ref is None, in {:?}", web_gl_canvas);
        let error_msg = format!(
            "Error: web_gl_canvas.canvas_ref is None, in {:?}",
            web_gl_canvas,
        );
        view! { <p>{error_msg}</p> }.into_any()
    }
}
