use eframe::egui;
use egui::ScrollArea;
use meval::eval_str;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> eframe::Result {
    // Set up window options with initial size.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]).with_resizable(false).with_fullscreen(false),
        ..Default::default()
    };

    // Variables to hold the input and result/output.
    let mut input = String::new();    // Holds the user input.
    let mut result = String::new();   // Holds the result of the calculation.
    let mut history: Vec<String> = vec![];
    let mut visible = true;
    let mut view_hist = false;

    // Run the app.
    eframe::run_simple_native("Calculator", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {

            // Heading for the app.
            ui.heading("Simple Calculator App");

            ScrollArea::vertical().show(ui, |ui| {

            // Input field for the user to type the equation.
            let input_label = ui.label("Input: ");
            ui.text_edit_singleline(&mut input)
                .labelled_by(input_label.id); // Bind the label to the input field.

            ui.group(|ui| {

            ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut visible, "Show Operators");
            if visible {

            ui.add(egui::Label::new("\n \n").wrap());

            // Button to insert the "+" character into the input field.
            if ui.button("Add +").clicked() {
                input.push('+'); // Append the "+" character when the button is clicked.
            }

            if ui.button("Subtract -").clicked() {
                input.push('-'); // Append the "-" character when the button is clicked.
            }

            if ui.button("Multiply x").clicked() {
                input.push('*'); // Append the "*" character when the button is clicked.
            }

            if ui.button("Divide ÷").clicked() {
                input.push('/'); // Append the "/" character when the button is clicked.
            }

            if ui.button("Power of ^").clicked() {
                input.push('^'); // Append the "^" character when the button is clicked.
            }

            if ui.button("Parentheses ()").clicked() {
                input.push_str("()"); // Append the "()" string when the button is clicked.
            }

            if ui.button("Square Root √").clicked() {
                input.push_str("√"); // Append the "√" string when the button is clicked.
            }

            if ui.button("Sin sin(θ)").clicked() {
                input.push_str("sin()"); // Append the "sin()" string when the button is
            }

            if ui.button("Tan tan(θ)").clicked() {
                input.push_str("tan()"); // Append the "sin()" string when the button is
            }

            if ui.button("Cos cos(θ)").clicked() {
                input.push_str("cos()"); // Append the "sin()" string when the button is
            }
        }
        })
        });

            // Button to evaluate the equation (perform the calculation).
            if ui.button("Calculate").clicked() {

                let input = insert_operator(&input);

                // Try to evaluate the equation if valid, otherwise display an error.
                match eval_str(&input) {
                    Ok(answer) => {
                        result = answer.to_string();
                        let entry = format!("{} = {}", input.clone(), result);
                        history.push(entry.clone());
                        append_to_history_file(&entry);
                    },
                    Err(_) => result = "Error".to_string(), // Show error if parsing fails.
                }

            }

            if ui.button("Clear").clicked() {
                input.clear();
                result.clear();
            }

            // Output field to display the result of the calculation.
            let result_label = ui.label("Answer: ");
            ui.text_edit_singleline(&mut result)
                .labelled_by(result_label.id); // Bind the label to the result field.

            
            ui.checkbox(&mut view_hist, "Show History");

            if view_hist{
                ui.label("History: ");
                ScrollArea::vertical().show(ui, |ui| {
                    for entry in &history {
                        ui.label(entry);
                    }
                    if ui.button("Clear History").clicked(){
                        OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open("history.txt")
                        .expect("Cannot open file");

                        history.clear();
                    }
                });
            }
        });
    });
    })
}

fn insert_operator(expression: &str) -> String {
    let mut modified = String::new();
    let chars: Vec<char> = expression.chars().collect();
    let mut prev_char = ' ';
    let mut in_sqrt = false;

    for &c in chars.iter() {
        // Handle implicit multiplication before '(' or '√'
        if prev_char.is_digit(10) && (c == '(' || c == '√') {
            modified.push('*'); // Insert '*' for implicit multiplication
        }

        // Replace '√' with "sqrt("
        if c == '√' {
            modified.push_str("sqrt(");
            in_sqrt = true; // We're inside a sqrt operation
        } else {
            // If we're inside a sqrt, check when to close the parentheses
            if in_sqrt {
                if c.is_digit(10) || c == '(' {
                    modified.push(c);
                } else {
                    // Close the sqrt parentheses before adding other operators
                    modified.push(')');
                    modified.push(c);
                    in_sqrt = false; // We're out of the sqrt operation
                }
            } else {
                modified.push(c);
            }
        }

        prev_char = c;
    }

    // If the expression ended inside a sqrt, close the parentheses
    if in_sqrt {
        modified.push(')');
    }

    modified
}

fn append_to_history_file(entry: &str) {
    // Open the file in append mode, create if it doesn't exist.
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("history.txt")
        .expect("Cannot open file");

    // Write the entry to the file.
    writeln!(file, "{}", entry).expect("Cannot write to file");
}