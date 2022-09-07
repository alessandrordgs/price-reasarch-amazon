use regex::Regex;

#[tokio::main]
async fn req() -> Result<String, reqwest::Error> {
    // let body = reqwest::get("https://www.amazon.com.br/dp/B08VKLCRYY/?coliid=I7USG42RRF337&colid=3ULTYET00I967&ref_=lv_ov_lig_dp_it&th=1")
    let body = reqwest::get("https://www.amazon.com.br/dp/B089DR3N7S/?coliid=I6CHPZH7H1Y5S&colid=3ULTYET00I967&psc=1&ref_=lv_ov_lig_dp_it_im")
        .await?;
    let text = body.text().await?;
    // println!("body = {:?}", text);

    Ok(text)
}

fn main() {
    let body = req();
    // let mut contains = Vec::new();
    let regex = Regex::new("space--><span class=\"a-price-whole\"").unwrap();
    let bodytest = match body {
        Ok(text) => text,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let results = bodytest.lines().filter(|line| regex.is_match(line));
    for line in results {
        // let span = line.split("span").collect();
        if regex.is_match(line) {
            let a: Vec<&str> = line.split(",").collect();
            let value: Vec<&str> = a[0].split("<span class=").collect();
            let last_value: Vec<&str> = value[1].trim().split("a-price-whole").collect();
            let current_value: Vec<&str> = last_value[1].split(">").collect();
            println!("{:?}", current_value[1])
        }
    }

    // for line in bodytest.lines() {
    //     if line.to_lowercase().contains("class=\"a-price-whole\"") {
    //        for lines in line.lines() {
    //         if lines.contains("<span class=\"a-offscreen\">") {
    //             let a = lines.chars().as_str();
    //             contains.push(lines)
    //         }
    //        }
    //     }
    // }
    // println!("Hello, world! {:?}", results);
}
