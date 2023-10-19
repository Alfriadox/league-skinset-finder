//! Yew components to build out the League Skinset Finder frontend. 

use std::{collections::HashMap, rc::Rc};

use enumflags2::BitFlags;
use yew::prelude::*;
use link::Link;
use crate::lanes::Lane;
use player::Player;

pub mod player;
pub mod link;

/// State persisted for each player in the frontend. 
#[derive(Clone, Debug)]
pub struct PlayerRecord {
    /// Hide/exclude this player from view and calculation. 
    pub exclude: bool,
    /// Player name (optional -- resolve with player number otherwise).
    pub name: Option<Rc<str>>,
    /// List of champs and what lanes for them. 
    pub champs: HashMap<Rc<str>, BitFlags<Lane>>
}

impl PlayerRecord {
    /// Create a new player with a given number and otherwise empty fields. 
    pub fn new(exclude: bool) -> Self {
        Self { exclude, name: None, champs: HashMap::new() }
    }
}

/// Messages that can be passed to the top-level app.
pub enum Msg {
    /// A players name has been updated. 
    PlayerNameUpdate {
        index: usize,
        new_name: String,
    },

    /// A player has been toggled into the calculations. 
    PlayerToggle {
        index: usize,
        state: bool
    }
}

/// The main component that the frontend is rendered as. 
pub struct App {
    /// The five players (max) in the league comp. 
    players: Vec<PlayerRecord>
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        // Create the list of players stored in this app.
        let mut players = Vec::with_capacity(5);
        // Add the default player.
        players.push(PlayerRecord::new(false));
        // Return
        App { players }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Handle a player updating their name. 
            Msg::PlayerNameUpdate { index, new_name } => {
                // Update the player name in this object's model. 
                self.players[index].name = if new_name.is_empty() { None } else { Some(new_name.into()) };

                // Test log message
                log::info!("{:#?}", self.players);
            },

            Msg::PlayerToggle { index, state } => unimplemented!(),
        }

        // Always return true to indicate the need for a re-render.
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="mt-3 card bg-light text-dark"> 
                    <div class="container p-4"> 
                        <p class="h1"> {"League of Legends skinset finder"} </p>
                        <p> {"This tool is used to find League of Legend team comps that share skins from the same skinset."} </p>
                        <p> 
                            {"I currently source my skin data from "} 
                            <Link href="https://leagueoflegends.fandom.com/wiki/Champion_skin/Skin_themes" open_in_new_tab={true} />
                            {", and my lane data from "}
                            <Link href="https://leagueoflegends.fandom.com/wiki/List_of_champions_by_draft_position" open_in_new_tab={true} />
                            {"."}
                        </p>
                        <p> {"Data was last updated from these sources on October 11th, 2023."} </p>
                        <p>
                            {"
                            I will try to keep this generally up to date with league skins and champions, but may not always
                            remember to update this every patch. If you notice that the date above is a long time ago, or there
                            are champs/skins missing, please let me know by filing an Issue report at 
                            "}
                            <Link href="https://github.com/Alfriadox/league-skinset-finder/issues" open_in_new_tab={true} /> {"."}
                        </p>
                    </div>
                </div>
                
                {
                    self.players.iter()
                        .enumerate()
                        .map(|(id, player)| {
                            html! { 
                                <Player player_number={id as u8 + 1} on_name_change={
                                    ctx.link().callback(move |new_name| {
                                        Msg::PlayerNameUpdate { index: id, new_name }
                                    })
                                } /> 
                            }
                        })
                        .collect::<Html>()
                }
            </>
        }
    }
}
