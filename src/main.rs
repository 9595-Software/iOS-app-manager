use clap::Parser;
use ios_app_manager::*;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    device_name: String,
    #[clap(short, long)]
    app_path: String,
    #[clap(short, long)]
    launch: bool,
}

fn main() {
    let args = Cli::parse();

    let device_id = get_device_id(&args.device_name);

    let installation_url = get_installation_url(&device_id, args.app_path);

    if args.launch {
        launch_app(device_id, installation_url);
    }
}
