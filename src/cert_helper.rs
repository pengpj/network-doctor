use chrono::{DateTime, NaiveDateTime, ParseError, TimeZone, Utc};

/// for i in `ls /var/lib/rancher/k3s/server/tls/*.crt`; do echo $i; openssl x509 -enddate -noout -in $i; done
/// 检查证书有效期
/// 如果证书已经过期，输出证书路径和过期时间
pub(crate) fn cert_check() {
    let tls_dir = "/var/lib/rancher/k3s/server/tls";
    println!("tls dir: {}", tls_dir);
    let entries = match std::fs::read_dir(tls_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("failed to read dir: {}", e);
            return;
        }
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("failed to read entry: {}", e);
                continue;
            }
        };
        let path = entry.path();
        let path = match path.to_str() {
            Some(path) => path,
            None => {
                eprintln!("failed to convert path to str");
                continue;
            }
        };

        // path ends with .crt
        if !path.ends_with(".crt") {
            continue;
        }

        // 检查证书有效期
        let output = match std::process::Command::new("openssl")
            .arg("x509")
            .arg("-enddate")
            .arg("-noout")
            .arg("-in")
            .arg(path)
            .output() {
            Ok(output) => output,
            Err(e) => {
                eprintln!("failed to execute process: {}", e);
                continue;
            }
        };
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            print!("{} =>\n{}", path, stdout);
            // 解析证书有效期 Jul  1 08:00:00 2022 GMT
            let parts: Vec<&str> = stdout.split('=').collect();
            if parts.len() != 2 {
                eprintln!("invalid output: {}", stdout);
                continue;
            }
            let date = parts[1].trim().to_string();
            // 解析时间 Jan 10 11:32:20 2026 GMT
            // let data = "Wed, 18 Feb 2015 23:16:09 GMT";
            let date = match parse_openssl_time(&date) {
                Ok(date) => date,
                Err(e) => {
                    eprintln!("failed to parse date: {}", e);
                    continue;
                }
            };
            // 检查是否已经过期
            let now = chrono::Utc::now();
            if date < now {
                eprintln!("证书已过期，expired: {}", date);
            }
            // 打印过期天数
            let days = date.signed_duration_since(now).num_days();
            println!("{} 天后过期", days);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("failed: {}", stderr);
            continue;
        }
    }
}

fn parse_openssl_time(time_str: &str) -> Result<DateTime<Utc>, ParseError> {
    // let time_str = "Jan 10 11:32:20 2020 GMT";
    // println!("time_str: {}", time_str);
    let format = "%b %e %H:%M:%S %Y %Z";
    let naive = NaiveDateTime::parse_from_str(time_str, format);
    Ok(TimeZone::from_utc_datetime(&Utc, &naive.unwrap()))
    // Ok(DateTime::<Utc>::from_utc(naive.unwrap(), Utc))
    // .map(|dt| dt.with_timezone(&Utc));
    // DateTime::parse_from_str(time_str, format)
    //     .map(|dt| dt.with_timezone(&Utc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cert_check() {
        cert_check();
    }
}