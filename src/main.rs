#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashMap;

use egui::{RichText, Color32};
use egui_extras::RetainedImage;
use rand::Rng;
use serde::Deserialize;
use serde_xml_rs::from_str;

fn main(){
    tracing_subscriber::fmt::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 500.0)),
        ..Default::default()
    };

    let my_app = MyApp::default();

    eframe::run_native(
        "Radijo megėjų egzamino testavimo programa",
        options,
        Box::new(|_cc| Box::new(my_app)),
    );
}

struct MyApp {
    started:bool,
    questions:Questions,
    current_question:usize,
    selected_answer:Option<usize>,
    viewing_answer:bool,
    images:HashMap<String,RetainedImage>,
    answered:Vec<usize>,
    correct:usize,
    answered_all:bool,
}
impl MyApp{
    fn get_new_question(&mut self) -> usize{
        let question = rand::thread_rng().gen_range(0..self.questions.0.len());
        if !self.answered.contains(&question){
            question
        }
        else{
            self.get_new_question()
        }
    }
}
impl Default for MyApp{
    fn default() -> Self {
        MyApp{
            started: false,
            questions:parse_questions(),
            current_question:0,
            selected_answer:None,
            viewing_answer:false,
            images:{
                let mut map = HashMap::new();
                map.insert("images/b1.gif".to_string(),RetainedImage::from_image_bytes(
                    "",include_bytes!("images/b1.gif"),
                ).unwrap());
                map.insert("images/b2.gif".to_string(),RetainedImage::from_image_bytes(
                    "",include_bytes!("images/b2.gif"),
                ).unwrap());
                map.insert("images/b3.gif".to_string(),RetainedImage::from_image_bytes(
                    "",include_bytes!("images/b3.gif"),
                ).unwrap());
                map.insert("images/b4.gif".to_string(),RetainedImage::from_image_bytes(
                    "",include_bytes!("images/b5.gif"),
                ).unwrap());
                map.insert("images/b5.gif".to_string(),RetainedImage::from_image_bytes(
                    "",include_bytes!("images/b5.gif"),
                ).unwrap());
                map.insert("images/b6.gif".to_string(),RetainedImage::from_image_bytes(
                    "",include_bytes!("images/b6.gif"),
                ).unwrap());
                map.insert("images/b7.gif".to_string(),RetainedImage::from_image_bytes(
                    "",include_bytes!("images/b7.gif"),
                ).unwrap());
                map.insert("images/b8.gif".to_string(),RetainedImage::from_image_bytes(
                    "",include_bytes!("images/b8.gif"),
                ).unwrap());
                map
            },
            answered:Vec::new(),
            correct: 0,
            answered_all:false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {



        egui::CentralPanel::default().show(ctx, |ui| {

            if !self.started{
                ui.heading("Sveiki atvykę į egzaminavimo platformą!");
                if ui.button("Pradėti").clicked(){
                    self.started = true;
                    self.viewing_answer = false;
                    self.current_question = rand::thread_rng().gen_range(0..self.questions.0.len());
                }
            }
            else{
                if !self.answered_all{

                    let question = &self.questions.0[self.current_question];
                    ui.heading(question.text.clone());
    
                    if !self.viewing_answer{
                        for i in 0..3{
                            if ui.radio(if let Some(id) = self.selected_answer {i==id} else{false},question.choice[i].clone()).clicked(){
                                self.selected_answer = Some(i);
                            }
                        }
                        if !question.image.is_empty(){
                            ui.add(
                                egui::Image::new(self.images[&question.image].texture_id(ctx),self.images[&question.image].size_vec2()/3.0)
                            );
                        }
    
                        if self.selected_answer.is_some(){
                            if ui.button("Atsakyti").clicked(){
                                self.viewing_answer = true;
                                self.answered.push(self.current_question);
                                if self.selected_answer.unwrap() == question.answer{
                                    self.correct+=1;
                                }
    
                            }
                        }
                        else{
                            ui.label(egui::RichText::new("Pasirinkite atsakymą").color(egui::Color32::RED));
                        }
        
                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui|{
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                                if ui.button("Praleisti").clicked(){
                                    self.current_question = self.get_new_question();
                                }
                            });
                        });
                    }
                    else{
                        let selected_answer = self.selected_answer.unwrap();
                        
                        if selected_answer == question.answer{
                            ui.label(RichText::new("Teisingai!").color(Color32::LIGHT_GREEN).size(30.0));
                        }
                        else{
                            ui.label(RichText::new("Neteisingai!").color(Color32::LIGHT_RED).size(30.0));
                            ui.label(RichText::new("Pasirinktas atsakymas:").color(Color32::LIGHT_RED));
                            ui.label(question.choice[selected_answer].clone());
                            ui.label(RichText::new("Teisingas atsakymas:").color(Color32::LIGHT_GREEN));
                            ui.label(question.choice[question.answer].clone());
                        }                           
                        if ui.button("Tęsti").clicked(){
                            if self.answered.len() == self.questions.0.len(){
                                self.selected_answer = None;
                                self.viewing_answer = false;
                                self.answered_all = true;
                            }
                            else{
                                self.current_question = self.get_new_question();
                                self.selected_answer = None;
                                self.viewing_answer = false;
                            }
                        }
                    }
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui|{
                        let percent = if self.answered.is_empty() {0.0} else {self.correct as f32 / self.answered.len() as f32 * 100.0};
                        ui.label(format!("Teisingai atsakyti klausimai: {} iš {} ({:.2}%)",self.correct,self.answered.len(),percent));
                    });
                }
                else{
                    ui.label(RichText::new("Jūs atsakėte į visus klausimus! Šaunu!").color(Color32::LIGHT_GREEN).size(30.0));
                    let percent = if self.answered.is_empty() {0.0} else {self.correct as f32 / self.answered.len() as f32 * 100.0};
                    ui.label(format!("Teisingai atsakyti klausimai: {} iš {} ({:.2}%)",self.correct,self.answered.len(),percent));
                    if ui.button("Bandyti iš naujo").clicked(){
                        self.answered.clear();
                        self.correct = 0;
                        self.answered_all = false;
                    }
                }
            }
        });
    }
}

#[derive(Debug, Deserialize)]
struct Questions(Vec<Question>);
#[derive(Debug, Deserialize)]
struct Question{
    text: String,
    choice: Vec<String>,
    answer: usize,
    #[serde(default)]
    image:String,
}
const QUESTIONS_XML:&str = include_str!("quiz.xml");
fn parse_questions() -> Questions{
    from_str(QUESTIONS_XML).unwrap()
}
