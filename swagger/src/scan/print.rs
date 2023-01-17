use super::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
pub const LEFT_PAD: usize = 40;
pub const TBL_LEN: usize = 190;
pub const URL_LEN: usize = 75;
pub fn print_active_alerts(checks: Vec<ActiveChecks>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Check", "Top Severity", "Alerts"]);
    for check in checks {
        table.add_row(vec![
            Cell::new(check.name()).add_attribute(Attribute::Bold),
            check.top_severity().printable(),
            check.alerts_text(),
        ]);
    }
    println!("{table}");
}
pub fn print_active_alerts_verbose(checks: Vec<ActiveChecks>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "Check",
            "Severity",
            "Description",
            "Location",
            "Certainty",
        ]);
    for check in checks {
        for alert in check.inner() {
            table.add_row(vec![
                Cell::new(check.name()).add_attribute(Attribute::Bold),
                alert.level.printable(),
                Cell::new(alert.description).add_attribute(Attribute::Bold),
                Cell::new(alert.location).add_attribute(Attribute::Bold),
                alert.certainty.printable(),
            ]);
        }
    }
    println!("{table}");
}
pub fn print_passive_alerts(checks: Vec<PassiveChecks>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Check", "Top Severity", "Alerts", "Description"]);
    for check in checks {
        table.add_row(vec![
            Cell::new(check.name()).add_attribute(Attribute::Bold),
            check.top_severity().printable(),
            check.alerts_text(),
            Cell::new(check.description()).add_attribute(Attribute::Bold),
        ]);
    }
    println!("{table}");
}
pub fn print_passive_alerts_verbose(checks: Vec<PassiveChecks>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Check", "Severity", "Description", "Location"]);
    for check in checks {
        for alert in check.inner() {
            table.add_row(vec![
                Cell::new(check.name()).add_attribute(Attribute::Bold),
                alert.level.printable(),
                Cell::new(alert.description).add_attribute(Attribute::Bold),
                Cell::new(trim_location(alert.location)).add_attribute(Attribute::Bold),
            ]);
        }
    }
    println!("{table}");
}
fn prep_param(param: &ParamForTable) -> (String, String, String, String, String, String, String) {
    let max = if let Some(m) = param.max {
        m.to_string()
    } else {
        "NULL".to_string()
    };
    let min = if let Some(m) = param.min {
        m.to_string()
    } else {
        "NULL".to_string()
    };
    let mut statuses = String::new();
    let mut dms = String::new();
    let mut eps = String::new();
    let mut parents = String::new();
    let mut children = String::new();
    for status in &param.statuses {
        statuses.push_str(status);
        statuses.push('\n');
    }
    for dm in &param.dms {
        dms.push_str(&format!("{dm:?}"));
        dms.push('\n');
    }
    for ep in &param.eps {
        eps.push_str(ep);
        eps.push('\n');
    }
    for parent in &param.parents {
        parents.push_str(parent);
        parents.push('\n');
    }
    for child in &param.children {
        children.push_str(child);
        children.push('\n');
    }
    (min, max, statuses, dms, eps, parents, children)
}
// NOT YET USABLE SINCE THERE IS NO WAY FOR MULTICOLORING ONE CELL
pub fn print_param_table(params: &Vec<ParamForTable>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "Name",
            "Type",
            "Statuses",
            "Delivery Methods",
            "Endpoints",
            "Parents",
            "Children",
            "Min-Max",
        ]);
    for param in params {
        let (min, max, statuses, dms, eps, parents, children) = prep_param(param);
        table.add_row(vec![
            Cell::new(param.name.clone()).add_attribute(Attribute::Bold),
            Cell::new(param.param_type.clone()).add_attribute(Attribute::Bold),
            Cell::new(statuses).add_attribute(Attribute::Bold),
            Cell::new(dms).add_attribute(Attribute::Bold),
            Cell::new(eps).add_attribute(Attribute::Bold),
            Cell::new(parents).add_attribute(Attribute::Bold),
            Cell::new(children).add_attribute(Attribute::Bold),
            Cell::new(format!("{min}-{max}")).add_attribute(Attribute::Bold),
        ]);
    }
    println!("{table}");
}
fn prep_ep(eps: &EpForTable) -> (String, String, String, String, String, String) {
    let mut q_p = String::new();
    let mut h_p = String::new();
    let mut r_b_p = String::new();
    let mut r_p = String::new();
    let mut statuses = String::new();
    let mut methods = String::new();
    for method in &eps.ops {
        methods.push_str(&method.to_string());
        methods.push('\n');
    }
    for status in &eps.statuses {
        statuses.push_str(status);
        statuses.push('\n');
    }
    for p in &eps.query_params {
        q_p.push_str(&p.to_string());
        q_p.push('\n');
    }
    for p in &eps.headers_params {
        h_p.push_str(p);
        h_p.push('\n');
    }
    for p in &eps.req_body_params {
        r_b_p.push_str(p);
        r_b_p.push('\n');
    }
    for p in &eps.res_params {
        r_p.push_str(p);
        r_p.push('\n');
    }
    (methods, q_p, h_p, r_b_p, r_p, statuses)
}
// NOT YET USABLE SINCE THERE IS NO WAY FOR MULTICOLORING ONE CELL
pub fn print_ep_table(eps: &Vec<EpForTable>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "Path",
            "Methods",
            "Query Params",
            "Header Params",
            "Body Params",
            "Response Params",
            "Statuses",
        ]);
    for ep in eps {
        let (methods, q_p, h_p, r_b_p, r_p, statuses) = prep_ep(ep);
        table.add_row(vec![
            Cell::new(ep.path.clone()).add_attribute(Attribute::Bold),
            Cell::new(methods).add_attribute(Attribute::Bold),
            Cell::new(q_p).add_attribute(Attribute::Bold),
            Cell::new(h_p).add_attribute(Attribute::Bold),
            Cell::new(r_b_p).add_attribute(Attribute::Bold),
            Cell::new(r_p).add_attribute(Attribute::Bold),
            Cell::new(statuses).add_attribute(Attribute::Bold),
        ]);
    }
    println!("{table}");
}
fn trim_location(loc: String) -> String {
    loc.replace("swagger root", "")
        .replace("swagger rooot", "")
        .replace("swagger", "")
        .replace("media type:application/json", "")
        .replace("response status", "status")
}
impl Level {
    pub fn printable(&self) -> Cell {
        match self {
            Self::Info => Cell::new("INFO")
                .fg(Color::Blue)
                .add_attribute(Attribute::Bold),
            Self::Low => Cell::new("LOW")
                .fg(Color::Yellow)
                .add_attribute(Attribute::Bold),
            Self::Medium => Cell::new("MEDIUM")
                .fg(Color::Rgb {
                    r: 255,
                    g: 167,
                    b: 38,
                })
                .add_attribute(Attribute::Bold),
            Self::High => Cell::new("HIGH")
                .fg(Color::Red)
                .add_attribute(Attribute::Bold),
            Self::Critical => Cell::new("CRITICAL")
                .fg(Color::Black)
                .add_attribute(Attribute::Bold),
        }
    }
}
impl Certainty {
    pub fn printable(&self) -> Cell {
        match self {
            Self::Low => Cell::new("LOW")
                .fg(Color::DarkGrey)
                .add_attribute(Attribute::Bold),
            Self::Medium => Cell::new("MEDIUM")
                .fg(Color::DarkGrey)
                .add_attribute(Attribute::Bold),
            Self::High => Cell::new("HIGH")
                .fg(Color::DarkGrey)
                .add_attribute(Attribute::Bold),
            Self::Certain => Cell::new("CERTAIN")
                .fg(Color::DarkGrey)
                .add_attribute(Attribute::Bold),
            Self::Passive => Cell::new(""),
        }
    }
}
/*
//pub const LEFT_PAD: usize = 40;
//pub const TBL_LEN: usize = 190;
//pub const URL_LEN: usize = 75;
pub fn print_checks_table<T>(checks: &[T])
where T:fmt::Display+Check{
    println!(
        "{:pad$}| RESULT | TOP SEVERITY | ALERTS  |DESCRIPTION\n{:-<table_len$}",
        "CHECK",
        "",
        pad = LEFT_PAD,
        table_len = TBL_LEN
    );
    for check in checks {
        println!("{}", check);
    }
}
pub fn print_failed_checks_table<T>(checks: &[T])
where T:fmt::Display+Check{
    println!(
        "{:pad$}| RESULT | TOP SEVERITY | ALERTS  |DESCRIPTION\n{:-<table_len$}",
        "CHECK",
        "",
        pad = LEFT_PAD,
        table_len = TBL_LEN
    );
    for check in checks {
        if check.result() == "FAILED" {
            println!("{}", check);
        }
    }
}
fn split_text_to_lines(string:&str)->Vec<String>{
    let mut new_vec = vec![];
    let mut new_str = String::new();
    let line_len = 75;
    let mut c = 0;
    for t in string.split(' '){
        if !t.trim().is_empty(){
            c+=t.len()+1;
            if c>line_len{
                c = t.len();
                new_str.pop();
                new_vec.push(new_str);
                new_str = format!(" {}",t);
            }else{
                new_str.push_str(&format!("{} ",t.trim()));
            }
        }
    }
    new_vec.push(new_str);
    new_vec
}
pub fn print_alerts_table(checks: &[PassiveChecks]) {
    println!(
        "{:pad$}| LEVEL   |{:75}|DESCRIPTION\n{:-<table_len$}",
        "CHECK",
        "LOCATION",
        pad = 30,
        table_len = TBL_LEN
    );
    for check in checks {
        if check.result() == "FAILED" {
            for alert in check.inner() {
                println!("{:pad$}|{}", check.name().cyan().bold(), alert, pad = 30)
            }
        }
    }
}

pub fn print_attack_alerts_table(checks: &[ActiveChecks]) {
    println!(
        "{:pad$}| SEVERITY | CERTAINTY |{:thing$}|DESCRIPTION\n{:-<table_len$}",
        "CHECK",
        "LOCATION",
        "",
        table_len = TBL_LEN,
        pad = 30,
        thing = URL_LEN
    );
    for check in checks {
        if check.result() == "FAILED" {
            for _ in check.inner() {
                // println!("{:pad$}|{}", check.name().cyan().bold(), alert, pad = 30)
                println!("{}",serde_json::to_string(&check).unwrap());
            }
        }
    }
}

impl fmt::Display for PassiveChecks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.result() == "PASSED" {
            write!(
                f,
                "{:pad$}| {} |    {:8}  |  {:5}  |{}\n{:-<table_len$}",
                self.name().bold(),
                self.result().green().bold().underline(),
                "NONE".blue().bold(),
                self.alerts_text(),
                self.description(),
                "",
                pad = LEFT_PAD,
                table_len = TBL_LEN
            )
        } else if self.result() == "FAILED" {
            write!(
                f,
                "{:pad$}| {} |    {}  |  {:5}  |{}\n{:-<table_len$}",
                self.name().bold(),
                self.result().red().bold().underline(),
                self.top_severity(),
                self.alerts_text(),
                self.description(),
                "",
                pad = LEFT_PAD,
                table_len = TBL_LEN
            )
        } else {
            write!(f, "")
        }
    }
}
impl fmt::Display for ActiveChecks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.result() == "PASSED" {
            write!(
                f,
                "{:pad$}| {} |    {:8}  |  {:5}  |{}\n{:-<table_len$}",
                self.name().bold(),
                self.result().green().bold().underline(),
                "NONE".blue().bold(),
                self.alerts_text(),
                self.description(),
                "",
                pad = LEFT_PAD,
                table_len = TBL_LEN
            )
        } else if self.result() == "FAILED" {
            write!(
                f,
                "{:pad$}| {} |    {}  |  {:5}  |{}\n{:-<table_len$}",
                self.name().bold(),
                self.result().red().bold().underline(),
                self.top_severity(),
                self.alerts_text(),
                self.description(),
                "",
                pad = LEFT_PAD,
                table_len = TBL_LEN
            )
        } else {
            write!(f, "")
        }
    }
}
impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.certainty==Certainty::Passive{
            let location = self
                .location
                .replace("swagger root", "")
                .replace("swagger rooot", "")
                .replace("swagger", "")
                .replace("media type:application/json", "")
                .replace("response status", "status");
            let mut string = String::new();
            let location = split_text_to_lines(&location);
            string.push_str(&format!(
                " {:10}|{:75}|  {}\n",
                self.level,
                location[0].bright_magenta().bold(),
                self.description.bright_red().bold(),
            ));
            for loc in location.iter().skip(1){
                string.push_str(&format!(
                    "{:30}|{:9}|{:75}|  {}\n",
                    "",
                    "",
                    loc.bright_magenta().bold(),
                    ""
                ));
            }
            string.push_str(&format!("\n{:-<190}",""));
            write!(f,"{}",string)
        }else{
            /*write!(
                f,
                "  {}| {}  |{:thing$}|  {:}\n{:-<table_len$}",
                self.level,
                self.certainty,
                self.location.bright_magenta().bold(),
                self.description.bright_red().bold(),
                "",
                thing=URL_LEN,
                table_len = TBL_LEN
            )*/
            write!(f,"")
        }
    }
}
impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "{:8}", "INFO".blue().bold()),
            Self::Low => write!(f, "{:8}", "LOW".yellow().bold()),
            Self::Medium => write!(f, "{:8}", "MEDIUM".truecolor(255, 167, 38).bold()),
            Self::High => write!(f, "{:8}", "HIGH".red().bold()),
            Self::Critical => write!(f, "{:8}", "CRITICAL".red().bold().blink()),
        }
    }
}
impl fmt::Display for Certainty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "{:8}", "LOW".bright_black().bold()),
            Self::Medium => write!(f, "{:8}", "MEDIUM".bright_black()/*.truecolor(255, 167, 38)*/.bold()),
            Self::High => write!(f, "{:8}", "HIGH".bright_black().bold()),
            Self::Certain => write!(f, "{:8}", "CERTAIN".bright_black().bold()),
            Self::Passive=> write!(f, ""),
        }
    }
}
*/
