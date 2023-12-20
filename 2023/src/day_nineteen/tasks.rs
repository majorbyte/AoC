use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Rule{
    rule: char,
    operator: char,
    value: u16,
    result: String
}

#[derive(Debug)]
struct Workflow{
    name: String,
    rules: Vec<Rule>
}

#[derive(Debug)]
struct WorkflowList(Vec<Workflow>);

impl WorkflowList{
    pub fn add_workflow(&mut self,input: &str){
        let p = input.split("{").collect::<Vec<&str>>();
        let name = p[0];
    
        let mut _p = p[1].split(",").collect::<Vec<&str>>();
        _p.reverse();
        let mut parts = VecDeque::from(_p);
    
        let mut rules:Vec<Rule> = vec![];
    
        while parts.len() > 0{
            let part = parts.pop_back().unwrap();
            if !part.contains(":") {
                rules.push(Rule{ rule:' ', operator:' ' , value:0, result:part.replace("}","") });
                continue;
            }
            let r = part.split(":").collect::<Vec<&str>>();
            let t = r[0].chars().collect::<Vec<char>>();
            let v = r[0][2..].parse::<u16>();
            if v.is_err() {panic!("oh no");}
            rules.push(Rule{ rule: t[0], operator: t[1], value:v.unwrap(), result:String::from(r[1])});
        }
    
        self.0.push(Workflow{name:String::from(name),rules});
    }

    pub fn get_workflow(&self, name: &str) -> Option<&Workflow> {
        self.0.iter().find(|wf| wf.name == name)
    }
}

impl Rule{
    pub fn is_ok(&self) -> bool{
        self.result == "A" || self.result ==  "R"
    }
}

#[derive(Debug)]
struct PartsList(Vec<Parts>);

#[derive(Debug)]
struct Parts(Vec<Part>);

#[derive(Debug)]
struct Part{
    part_type:char,
    value:u16
}

impl PartsList{
    pub fn add_parts(&mut self, input: &str){
        let parts = input.replace("{","").replace("}","").split(",").map(|p| Part::create(p)).collect::<Vec<Part>>();
        self.0.push(Parts{0:parts});
    }

    pub fn get_accepted(&self, workflows: &WorkflowList) -> Vec<&Parts> {

        let mut accepted: Vec<&Parts> = vec![];
        self.0.iter().for_each(|parts|{
            let workflow = workflows.get_workflow("in").unwrap();

            let result = self.get_result(workflows, workflow, parts);
            if result.is_some() {
                accepted.push(result.unwrap());
            }
        });
        
        accepted
    }

    fn get_result<'a>(&'a self, workflows: &WorkflowList, workflow: &Workflow , parts: &'a Parts) -> Option<&Parts> {

        let mut result: &Part;

        for rule in &workflow.rules {
            let p = parts.0.iter().find(|c|  rule.rule == c.part_type);
            if !p.is_some() {continue;}
    
            let part = p.unwrap();
            match rule.operator {
                '>' => 
                    if part.value > rule.value && rule.is_ok() {
                        if rule.result == "A" { return Some(parts);}
                        return None;
                    } else if part.value > rule.value {
                        let wf = workflows.get_workflow(&rule.result);
                        if wf.is_none() {panic!("workflow {} not found", rule.result);}
                        return self.get_result(workflows, wf.unwrap(), parts);
                    }  
                '<' => 
                    if part.value < rule.value && rule.is_ok(){
                        if rule.result == "A" { return Some(parts);}
                        return None;
                    } else if part.value < rule.value {
                        let wf = workflows.get_workflow(&rule.result);
                        if wf.is_none() {panic!("workflow {} not found", rule.result);}
                        return self.get_result(workflows, wf.unwrap(), parts);

                    },
                    _ => {panic!("unknow operator {}", rule.operator);}
            }
        }
        let rule = workflow.rules.last().unwrap();
        if !rule.is_ok() {
            let wf = workflows.get_workflow(&rule.result);
            return self.get_result(workflows, wf.unwrap(), parts);
        }
        if rule.result == "A" {return Some(parts)}
        None
    }

}

impl Parts{
    pub fn sum(&self) -> u128{
        self.0.iter().map(|p| p.value as u128 ).sum()
    }
}

impl Part{
    pub fn create(input: &str) -> Part{
        let p = input.split("=").collect::<Vec<&str>>();
        Part{ part_type: p[0].chars().collect::<Vec<char>>()[0], value: p[1].parse::<u16>().unwrap()}
    }
}

pub fn task() {
    // Create a path to the desired file
    let path = Path::new("./src/day_nineteen/input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => { 
            task_1(&s);
            //task_2(&s);
         }
    }
}

fn task_1(data: &str){
    let input = data.split("\r\n\r\n").collect::<Vec<&str>>();

    let mut workflow_list = WorkflowList{0:vec![]};
    let mut parts_list = PartsList{0:vec![]};
    input[0].split("\r\n").for_each(|line| workflow_list.add_workflow( line));
    input[1].split("\r\n").for_each(|line| parts_list.add_parts( line));
    
    let accepted = parts_list.get_accepted(&workflow_list);
    
    println!("{}", accepted.iter().map(|a| a.sum()).sum::<u128>())
}
