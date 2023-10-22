//! Yew components to build out the League Skinset Finder frontend. 

use std::{rc::Rc, cell::RefCell};

use enumflags2::BitFlags;
use yew::prelude::*;
use link::Link;
use crate::lanes::Lane;
use player::Player;
use yew_icons::{Icon, IconId};
use serde::{Serialize, Deserialize};

mod player;
mod link;
mod checkbox;
mod button;

/// State persisted for each player in the frontend. 
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerRecord {
    /// Hide/exclude this player from view and calculation. 
    pub exclude: bool,
    /// Player name (optional -- resolve with player number otherwise).
    pub name: Option<AttrValue>,
    /// List of champs and what lanes for them. This is stored in an [`Rc`]'d [`RefCell`] for easy cloning/sharing
    /// with interior mutability. 
    pub champs: Rc<RefCell<Vec<(AttrValue, BitFlags<Lane>)>>>
}

impl PlayerRecord {
    /// Create a new player with a given number and otherwise empty fields. 
    pub fn new(exclude: bool) -> Self {
        Self { exclude, name: None, champs: Rc::new(RefCell::new(Vec::with_capacity(160))) }
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
    },

    /// Add a player on to the end of the list.
    AddPlayer,

    /// Remove a player from the list.
    RemovePlayer {
        /// The index of the player to remove. 
        player_index: usize,
    },

    // /// Add a champion to a player's list of playable champions.
    // /// This message gets re-sent when a player changes the lanes for a champ too, 
    // /// so be ready to handle that. 
    // AddChampToPlayer {
    //     /// The index of the player in the players list.
    //     player_index: usize,
    //     /// The name of the champion being added to this player. 
    //     champ_name: String,
    //     /// The lanes that the player is willing to play this champion.
    //     lanes: BitFlags<Lane>
    // },

    // /// Removes a champ from a player's list of playable champions.
    // RemoveChampFromPlayer {
    //     /// The index of the player to remove the champ from.
    //     player_index: usize,
    //     /// The name of the champ to remove. 
    //     champ_name: String,
    // },

    /// When a player updates their champ list this component has to re-render. 
    PlayerChampListUpdate,
}

/// The main component that the frontend is rendered as. 
#[derive(Debug)]
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

            Msg::AddPlayer => if self.players.len() <= 5 { self.players.push(PlayerRecord::new(false)) }

            Msg::RemovePlayer { player_index } => if self.players.len() >= 1 { self.players.remove(player_index); }

            // Msg::AddChampToPlayer { player_index, champ_name, lanes } => {
            //     // Mutably borrow the player's champs list. 
            //     let mut champs_borrow = self.players[player_index].champs.borrow_mut();
            //     // Check if the champ is in the list already. 

            //     // Set the champ's entry's lanes (upsert). 
            //     *champs_borrow.entry(champ_name.into()).or_default() = lanes;
            // }

            // Msg::RemoveChampFromPlayer { player_index, champ_name } => {
            //     self.players[player_index].champs.borrow_mut().remove(champ_name.as_str());
            // }

            // No-op here except for the re-render at the end. 
            Msg::PlayerChampListUpdate => {},
        }

        // Always return true to indicate the need for a re-render.
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Resolve whether any players can be removed currently. 
        let enable_player_removal = self.players.len() > 1;

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
                                <Player 
                                    {id}
                                    name={player.name.clone()}
                                    champs={player.champs.clone()}

                                    on_name_change={
                                        ctx.link().callback(move |new_name| {
                                            Msg::PlayerNameUpdate { index: id, new_name }
                                        })
                                    }

                                    enable_remove={enable_player_removal}
                                    on_remove={
                                        ctx.link().callback(move |_| {
                                            Msg::RemovePlayer { player_index: id }
                                        })
                                    }

                                    on_champ_list_update={ ctx.link().callback(move |_| Msg::PlayerChampListUpdate) }
                                /> 
                            }
                        })
                        .collect::<Html>()
                }

                // Block button to add a player. 
                <div class={"d-grid gap-2 mt-2"}> 
                    <button 
                        type={"button"} 
                        class={"btn btn-success"}
                        disabled={self.players.len() == 5}

                        // On-click handler to add a player.
                        onclick={
                            ctx.link().callback(move |_| {
                                Msg::AddPlayer
                            })
                        }
                    >
                        <Icon icon_id={IconId::BootstrapPersonAdd} /> {" Add Player"}
                    </button>
                </div>
            </>
        }
    }
}
