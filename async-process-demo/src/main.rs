use anyhow::Ok;
use async_process::{Command, Stdio};
use futures_lite::{io::BufReader, prelude::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut child = Command::new("ping")
    .arg("127.0.0.1")
    .stdout(Stdio::piped())
    .spawn()?;

    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let mut count = 0;
    println!("pid:{}",child.id());
    while let Some(line) = lines.next().await {
        println!("{}", line?);
        count = count +1;

        if count > 10 {
            child.kill().unwrap();
        }
    }

    let mut child = Command::new("ping")
        .arg("-c 5")
        .arg("127.0.0.1")
        .stdout(Stdio::piped())
        .spawn()?;
    let mut out_str = String::new();
    child.stdout.as_mut().unwrap().read_to_string(&mut out_str)
     .await?;
    println!("wait for child process");
    let exit_status =  child.status().await?;

    println!("child exit code:{}",exit_status);
    println!("output:{}",out_str);

    Ok(())
}
