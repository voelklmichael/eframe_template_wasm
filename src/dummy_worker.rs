use gloo_worker::Registrable;
fn main() {
    eframe_template::webworker::WebWorker::registrar().register();
}
