use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::data::Experience;

#[derive(Properties, PartialEq)]
pub struct ExperienceSectionProps {
    pub experiences: Vec<Experience>,
    pub error: Option<String>,
}

#[function_component]
pub fn ExperienceSection(props: &ExperienceSectionProps) -> Html {
    html! {
        <div class="max-w-6xl w-full mb-4 mx-auto projects-section flex flex-col items-center" style="font-family: 'Space Mono', monospace;">
            <h1 class="text-2xl font-bold mb-3 text-center" style="color: #08504B;">{"Work Experience"}</h1>

            {
                if let Some(error_message) = &props.error {
                    html! {
                        <div class="w-full bg-red-50 border-l-4 border-red-400 text-red-700 p-4 mb-6 rounded-md">
                            <p class="font-medium text-center">{error_message}</p>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            <div class="space-y-4 flex flex-col items-center w-full">
                {props.experiences.iter().map(|exp| {
                    html! {
                        <div class="bg-black border border-gray-700 rounded-lg shadow-md p-6 transform transition-all duration-300 hover:shadow-xl hover:-translate-y-1 bg-gradient-to-br from-gray-900 via-black to-gray-800"
                            style="font-family: 'Space Mono', monospace; color: white; width: 80%;">
                            <div class="flex justify-between items-center mb-4">
                                <div class="flex items-center">
                                    <h3 class="text-xl font-semibold text-white mr-4" style="color: #718096; font-style: italic; text-decoration: underline; line-height: 1.6; letter-spacing: 0.05em;">
                                        {&exp.company}
                                    </h3>
                                    <p class="font-semibold" style="font-family: 'Space Mono', monospace; color: #08504B;">
                                        {&exp.position}
                                    </p>
                                </div>
                                <span class="text-sm text-gray-300 bg-gray-800 px-3 py-1 rounded-full" style="font-family: 'Space Mono', monospace; color: #718096;">

                                    {&exp.period}
                                </span>
                            </div>
                            <div class="mt-2 text-gray-300" style="display: -webkit-box; -webkit-line-clamp: 5; color:#718096;  -webkit-box-orient: vertical; overflow: hidden; text-overflow: ellipsis; font-family: 'Space Mono', monospace;">
                                {&exp.description}
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
