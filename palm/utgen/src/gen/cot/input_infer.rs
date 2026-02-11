use super::Prompt;
use super::LLM;
use log::{error, info};
use rand::Rng;
use tokio::time::{sleep, Duration};

fn postprocess(inputs: &mut Vec<String>) {
    for input in inputs.iter_mut() {
        *input = input.replace("```rust", "").replace("```", "");
    }
}

pub async fn gen_input_range(pt_info: &Prompt, conds: &Vec<String>) -> Option<String> {
    let llm = LLM::new().unwrap();
    let system_pt = &pt_info.system_pt;
    let static_pt = &pt_info.static_pt;

    let user_pt = static_pt.clone() + &conds.join("");
    let mut retry = 0;
    let mut answers = None;
    while retry < 3 {
        if retry != 0 {
            let random_secs = {
                let mut rng = rand::rng();
                rng.random_range(10..=30)
            };
            sleep(Duration::from_secs(random_secs)).await;
        }
        let result = llm.fetch_answer(Some(&system_pt), &user_pt, 1, false).await;
        if result.is_ok() {
            answers = Some(result.unwrap());
            break;
        }
        error!("{}. Retrying...", result.unwrap_err());
        retry += 1;
    }
    if answers.is_none() {
        error!("Failed to fetch answer.");
        return None;
    }
    let mut answers = answers.unwrap();
    assert!(answers.len() == 1);

    postprocess(&mut answers);
    // info!("Answers: {:?}", answers);

    let mut answer = answers.pop().unwrap();
    if !answer.ends_with('\n') {
        answer.push('\n');
    }

    Some(answer)
}
