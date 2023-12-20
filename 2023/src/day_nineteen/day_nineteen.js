const fs = require('fs');

const node = data => {
    //px{a<2006:qkq,m>2090:A,rfg}

    let p = data.split("{");
    const name = p[0];
    
    let parts = p[1].split(",").reverse();
    let rules = [];
    while (parts.length > 0){
        let part = parts.pop();
        if (part.indexOf(":") === -1) {
            rules.push(part.replace("}",""));
            continue;
        }
        let r = part.split(":");
        rules.push({ rule:r[0], result:r[1]});
    }

    return {name, rules};
}

let processes;

const map = data => {
    let input = data.split("\r\n\r\n")
    processes = input[0].split("\r\n").map(node);

    const accepted = [];
    for (let line of input[1].split("\r\n")){

        let cmds = line.replace("{","").replace("}","").split(",");

        let workflow = get_workflow("in");

        let result = get_result(workflow, cmds);
        if (result == 'A'){
            accepted.push(cmds);
        }


    }
    console.dir(accepted);

    let sum =  accepted.reduce((p,c) => p +=c.reduce((n,m) => n += Number(m.substr(2)),0 ),0);
    console.log(sum);
}


const get_workflow = (name) =>  processes.find(p => p.name == name);


const get_result = (workflow, cmds) => {
    console.dir(workflow);

    console.dir(cmds);

    let result = undefined;
    for (let rule of workflow.rules){
        let cmd = cmds.find(c => rule.hasOwnProperty("rule") && rule.rule.indexOf(c[0])> -1)
        if (!cmd) continue;

        let c = cmd.split("=");
        let op = rule.rule[1];
        let val = Number(rule.rule.substr(2));

        switch (op){
            case ">": 
                if (Number(c[1]) > val ) result =  rule.result == 'R' || rule.result == 'A' ? rule.result : get_result(get_workflow(rule.result),cmds);
                break;
            case "<": 
                if (Number(c[1]) < val ) result =  rule.result == 'R' || rule.result == 'A' ? rule.result : get_result(get_workflow(rule.result),cmds);
                break;
        }

        if (result == 'A' || result == 'R') break;
    }
    if (result == undefined) {
        result = workflow.rules[workflow.rules.length-1];
        if (result != 'A' && result != 'R') result = get_result(get_workflow(result),cmds);
    }

    console.log(result);

    return result
}












fs.readFile('./input.txt', 'utf-8', (_,data) => map(data));