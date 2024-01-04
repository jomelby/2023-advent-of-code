use std::collections::HashSet;

use advent2023::advent_utils::get_lines_from_filepath;

pub struct Workflow {
    id: String,
    rules: Vec<Rule>,
    no_match_return: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Rule {
    operator: char,
    operands: i128,
    part_variable: char,
    return_value: String,
}

impl Rule {
    fn from_line(line: &str) -> Rule {
        let return_value = line.split(":").nth(1).unwrap().to_string();
        let part_variable = line.chars().nth(0).unwrap();
        let operator = line.chars().nth(1).unwrap();
        let mut operands_raw: Vec<char> = Vec::new();
        for char_ix in 2..line.chars().count() {
            if line.chars().nth(char_ix).unwrap() == ':' {
                break;
            }
            operands_raw.push(line.chars().nth(char_ix).unwrap());
        }
        let operands = operands_raw
            .iter()
            .collect::<String>()
            .parse::<i128>()
            .unwrap();
        Rule {
            operator,
            operands,
            part_variable,
            return_value,
        }
    }
}

impl Rule {
    fn evaluate(&self, part: &Part) -> bool {
        let mut part_operand = 0;
        match self.part_variable {
            'x' => part_operand = part.x,
            'm' => part_operand = part.m,
            'a' => part_operand = part.a,
            's' => part_operand = part.s,
            _ => panic!("Invalid part variable"),
        }

        match self.operator {
            '<' => part_operand < self.operands,
            '>' => part_operand > self.operands,
            '=' => part_operand == self.operands,
            _ => panic!("Invalid operator"),
        }
    }
}

impl Workflow {
    fn from_line(line: &String) -> Workflow {
        let id = line.split("{").next().unwrap().to_string();
        let rules_raw: String = line.split("{").nth(1).unwrap().to_string().replace("}", "");
        let rules_raw_split: Vec<&str> = rules_raw.split(",").collect();
        let mut rules = Vec::new();
        let no_match_return = rules_raw_split[rules_raw_split.len() - 1].to_string();
        for (ix, raw_rule) in rules_raw_split.iter().enumerate() {
            if ix == rules_raw_split.len() - 1 {
                break;
            }
            let rule = Rule::from_line(raw_rule);
            rules.push(rule);
        }
        Workflow {
            id,
            rules,
            no_match_return,
        }
    }
}

impl Workflow {
    fn evaluate(&self, part: &Part) -> String {
        for rule in &self.rules {
            if rule.evaluate(part) {
                return rule.return_value.clone();
            }
        }
        self.no_match_return.clone()
    }
}

fn run_workflows(current_workflow: &Workflow, workflows: &Vec<Workflow>, part: &Part) -> String {
    for rule in current_workflow.rules.iter() {
        if rule.evaluate(part) {
            if rule.return_value == "A" || rule.return_value == "R" {
                return rule.return_value.as_str().to_string();
            }
            let next_workflow = workflows
                .iter()
                .find(|workflow| workflow.id == rule.return_value)
                .unwrap();
            return run_workflows(next_workflow, workflows, part);
        }
    }
    if current_workflow.no_match_return == "A" || current_workflow.no_match_return == "R" {
        return current_workflow.no_match_return.clone();
    } else {
        return run_workflows(
            workflows
                .iter()
                .find(|workflow| workflow.id == current_workflow.no_match_return)
                .unwrap(),
            workflows,
            part,
        );
    }
}

#[derive(PartialEq, Debug)]
pub struct Part {
    x: i128,
    m: i128,
    a: i128,
    s: i128,
}

impl Part {
    fn from_line(line: &String) -> Part {
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        let line_split: Vec<&str> = line[1..(line.len() - 1)].split(",").collect();
        for (ix, part) in line_split.iter().enumerate() {
            match ix {
                0 => x = part[2..].parse::<i128>().unwrap(),
                1 => m = part[2..].parse::<i128>().unwrap(),
                2 => a = part[2..].parse::<i128>().unwrap(),
                3 => s = part[2..].parse::<i128>().unwrap(),
                _ => panic!("Too many parts in line"),
            }
        }
        Part { x, m, a, s }
    }
}

impl Part {
    fn sum(&self) -> i128 {
        self.x + self.m + self.a + self.s
    }
}

fn part_one(lines: &Vec<String>) -> i128 {
    let mut answer = 0;
    let workflow_lines = lines.iter().take_while(|line| **line != "".to_string());
    let mut workflows = Vec::new();
    for line in workflow_lines {
        workflows.push(Workflow::from_line(line));
    }
    let part_lines = lines.iter().skip_while(|line| **line != "".to_string());
    let mut parts = Vec::new();
    for (ix, line) in part_lines.enumerate() {
        if ix == 0 {
            continue;
        }
        parts.push(Part::from_line(line));
    }
    let starting_workflow = workflows
        .iter()
        .find(|workflow| workflow.id == "in")
        .unwrap();
    for part in parts.iter() {
        let result = run_workflows(starting_workflow, &workflows, part);
        if result == "A" {
            answer += part.sum();
        }
    }
    answer
}

fn all_possible_values() -> HashSet<usize> {
    let mut possible_values = HashSet::new();
    for i in 1..=4000 {
        possible_values.insert(i);
    }
    possible_values
}

fn part_two(lines: &Vec<String>) -> usize {
    let workflow_lines = lines.iter().take_while(|line| **line != "".to_string());
    let mut workflows = Vec::new();
    for line in workflow_lines {
        workflows.push(Workflow::from_line(line));
    }
    let mut possible_a_values = all_possible_values();
    let mut possible_m_values = all_possible_values();
    let mut possible_x_values = all_possible_values();
    let mut possible_s_values = all_possible_values();
    for workflow in workflows.iter() {
        for rule in workflow.rules.iter() {
            if rule.return_value == "R" {
                let mut values_to_remove: Vec<usize> = Vec::new();
                let all_possible_values = all_possible_values();
                if rule.operator == '<' {
                    for value in all_possible_values.iter() {
                        if *value < rule.operands as usize {
                            values_to_remove.push(*value);
                        }
                    }
                } else {
                    for value in all_possible_values.iter() {
                        if *value > rule.operands as usize {
                            values_to_remove.push(*value);
                        }
                    }
                }
                if rule.part_variable == 'a' {
                    for value in values_to_remove {
                        possible_a_values.remove(&value);
                    }
                } else if rule.part_variable == 'm' {
                    for value in values_to_remove {
                        possible_m_values.remove(&value);
                    }
                } else if rule.part_variable == 'x' {
                    for value in values_to_remove {
                        possible_x_values.remove(&value);
                    }
                } else if rule.part_variable == 's' {
                    for value in values_to_remove {
                        possible_s_values.remove(&value);
                    }
                }
            }
        }
    }
    println!(
        "a: {}, m: {}, x: {}, s: {}",
        possible_a_values.len(),
        possible_m_values.len(),
        possible_x_values.len(),
        possible_s_values.len()
    );
    possible_a_values.len()
        * possible_m_values.len()
        * possible_x_values.len()
        * possible_s_values.len() as usize
}

fn main() {
    let lines = get_lines_from_filepath("data/day_19_input.txt");
    println!("{}", part_one(&lines));
}

#[test]
fn test_part_one() {
    let lines = get_lines_from_filepath("data/day_19_test_input.txt");
    assert_eq!(part_one(&lines), 19114);
}

#[test]
fn test_part_two() {
    let lines = get_lines_from_filepath("data/day_19_test_input.txt");
    assert_eq!(part_two(&lines), 167409079868000);
}

#[test]
fn test_workflow_from_line() {
    let line = "px{a<2006:qkq,m>2090:A,rfg}".to_string();
    let workflow = Workflow::from_line(&line);
    assert_eq!(workflow.id, "px");
    assert_eq!(workflow.rules.len(), 2);
    assert_eq!(workflow.rules[0].operator, '<');
    assert_eq!(workflow.rules[0].operands, 2006);
    assert_eq!(workflow.rules[0].part_variable, 'a');
    assert_eq!(workflow.rules[0].return_value, "qkq");
    assert_eq!(workflow.no_match_return, "rfg")
}

#[test]
fn test_part_from_line() {
    let line = "{x=787,m=2655,a=1222,s=2876}".to_string();
    let part = Part::from_line(&line);
    assert_eq!(
        part,
        Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876
        }
    );
}

#[test]
fn test_rule_evaluate() {
    let rule = Rule {
        operator: '<',
        operands: 2006,
        part_variable: 'a',
        return_value: "qkq".to_string(),
    };
    let part = Part {
        x: 787,
        m: 2655,
        a: 1222,
        s: 2876,
    };
    assert_eq!(rule.evaluate(&part), true);
}
