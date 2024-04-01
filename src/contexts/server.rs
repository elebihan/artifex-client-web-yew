//
// Copyright (C) 2023 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::prelude::*;

pub const DEFAULT_PORT: u16 = 50051;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Server {
    pub url: String,
}

impl Reducible for Server {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Server { url: action }.into()
    }
}

pub type ServerContext = UseReducerHandle<Server>;

#[derive(Debug, PartialEq, Properties)]
pub struct ServerProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ServerProvider(props: &ServerProviderProps) -> Html {
    let location = use_location();
    let server = use_reducer(|| Server {
        url: format!("http://{}:{}", location.hostname, DEFAULT_PORT),
    });

    html! {
        <ContextProvider<ServerContext> context={server}>
            {props.children.clone()}
        </ContextProvider<ServerContext>>
    }
}
