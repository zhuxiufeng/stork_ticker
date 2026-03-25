use reqwest::Client;
use encoding_rs::GBK;
use crate::app::StockInfo;

pub async fn fetch_stocks(client: &Client, symbols: &[String]) -> Result<Vec<StockInfo>, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("http://hq.sinajs.cn/list={}", symbols.join(","));
    let res = client.get(&url).header("Referer", "https://finance.sina.com.cn").send().await?;
    let bytes = res.bytes().await?;
    
    // Sina Finance returns GBK encoded text
    let (decoded, _, _) = GBK.decode(&bytes);
    let mut stocks = Vec::new();

    for line in decoded.lines() {
        if line.is_empty() { continue; }
        
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() < 2 { continue; }
        
        let var_name = parts[0].trim();
        let symbol = var_name.replace("var hq_str_", "").replace("str_", "");
        
        let data_str = parts[1].trim().trim_matches(';').trim_matches('"');
        let fields: Vec<&str> = data_str.split(',').collect();
        if fields.len() < 3 { continue; }

        let mut name = String::new();
        let mut price = 0.0;
        let mut change_amount = 0.0;
        let mut change_percent = 0.0;

        if symbol.starts_with("gb_") {
            // US stocks
            name = fields[0].to_string();
            price = fields.get(1).unwrap_or(&"0").parse().unwrap_or(0.0);
            change_percent = fields.get(2).unwrap_or(&"0").parse().unwrap_or(0.0);
            change_amount = fields.get(4).unwrap_or(&"0").parse().unwrap_or(0.0);
        } else if symbol.starts_with("rt_hk") {
            // HK stocks
            name = fields[0].to_string();
            price = fields.get(6).unwrap_or(&"0").parse().unwrap_or(0.0);
            change_amount = fields.get(7).unwrap_or(&"0").parse().unwrap_or(0.0);
            change_percent = fields.get(8).unwrap_or(&"0").parse().unwrap_or(0.0);
        } else if symbol.starts_with("hf_") {
            // Futures (Gold)
            price = fields.get(0).unwrap_or(&"0").parse().unwrap_or(0.0);
            let prev_close: f64 = fields.get(7).unwrap_or(&"0").parse().unwrap_or(0.0);
            name = fields.get(13).unwrap_or(&symbol.as_str()).to_string();
            if price == 0.0 {
                price = prev_close;
            }
            if prev_close > 0.0 {
                change_amount = price - prev_close;
                change_percent = (change_amount / prev_close) * 100.0;
            }
        } else {
            // A-shares
            name = fields[0].to_string();
            price = fields.get(3).unwrap_or(&"0").parse().unwrap_or(0.0);
            let prev_close: f64 = fields.get(2).unwrap_or(&"0").parse().unwrap_or(0.0);
            if price == 0.0 {
                price = prev_close;
            }
            if prev_close > 0.0 {
                change_amount = price - prev_close;
                change_percent = (change_amount / prev_close) * 100.0;
            }
        }

        stocks.push(StockInfo {
            symbol,
            name,
            price,
            change_amount,
            change_percent,
        });
    }
    
    Ok(stocks)
}
