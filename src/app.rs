use std::collections::{HashMap, BTreeMap};

use battlebit_api::{ServerData, BBApi, Gamemode};

use yew::prelude::*;
use gloo::timers::callback::Timeout;


use ybc::TileCtx::{Ancestor, Child, Parent};

pub enum Msg {
    UpdateData,
    Updated(Vec<ServerData>),
    UpdateFailed,
}

pub struct App {
    server_data: Vec<ServerData>,
    timer_handle: Option<Timeout>,
}

impl App {
    fn region_count(&self) -> HashMap<String, usize> {
        self.server_data.iter().fold(HashMap::new(), |mut counts, server| {
            let region = server.region().to_string();

            match counts.get_mut(&region) {
                Some(val) => *val += 1,
                None => { counts.insert(region, 1usize); },
            }

            counts
        })
    }

    fn map_count(&self) -> HashMap<String, usize> {
        self.server_data.iter().fold(HashMap::new(), |mut counts, server| {
            match counts.get_mut(server.map()) {
                Some(val) => *val += 1,
                None => { counts.insert(server.map().clone(), 1usize); },
            }
            counts
        })
    }

    fn player_count(&self) -> (usize, usize) {
        self.server_data.iter().fold((0, 0), |mut counts, server| {
            counts.0 += *server.player_count() as usize;
            counts.1 += *server.queued_player_count() as usize;

            counts
        })
    }

    fn gamemode_count(&self) -> HashMap<String, usize> {
        fn gamemode_to_string(gamemode: &Gamemode) -> String {
            match gamemode {
                Gamemode::InfanteryConquest => String::from("Infantery Conquest"),
                Gamemode::TeamDeathmatch => String::from("Team Deathmatch"),
                Gamemode::CaptureTheFlag => String::from("Capture The Flag"),
                Gamemode::VoxelFortify => String::from("Voxel Fortify"),
                Gamemode::VoxelTrench => String::from("Voxel Trench"),
                Gamemode::FreeForAll => String::from("Free For All"),
                Gamemode::Gamemode19 => String::from("Gamemode 19"),
                _ => gamemode.to_string()
            }
        }

        self.server_data.iter().fold(HashMap::new(), |mut counts, server| {
            let gamemode = gamemode_to_string(server.gamemode());

            match counts.get_mut(&gamemode) {
                Some(val) => *val += 1,
                None => { counts.insert(gamemode, 1usize); },
            }

            counts
        })
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        ctx.link().send_message(Msg::UpdateData);

        Self {
            server_data: Vec::new(),
            timer_handle: None,
        }
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateData => {
                ctx.link().send_future(async {
                    let bbapi = BBApi::new();

                    match bbapi.server_list().await {
                        Ok(data) => Msg::Updated(data),
                        Err(_) => Msg::UpdateFailed
                    }
                });

                false
            },
            Msg::Updated(data) => {
                self.server_data = data;

                let handle = {
                    let link = ctx.link().clone();
                    Timeout::new(60_000, move || link.send_message(Msg::UpdateData))
                };

                self.timer_handle = Some(handle);

                true
            },
            Msg::UpdateFailed => {
                false
            },
        }
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> Html {
        let maps = self.map_count()
            .into_iter()
            .map(|(k,v)| (v,k))
            .collect::<BTreeMap<usize, String>>()
            .into_iter()
            .rev()
            .map(|(count, item)| {
                html!{ <> {format!("{item} ({count})")} <br/> </> }
            })
            .collect::<Vec<Html>>();

        let gamemodes = self.gamemode_count()
            .into_iter()
            .map(|(k,v)| (v,k))
            .collect::<BTreeMap<usize, String>>()
            .into_iter()
            .rev()
            .map(|(count, item)| {
                html!{ <> {format!("{item} ({count})")} <br/> </> }
            })
            .collect::<Vec<Html>>();

        let regions = self.region_count()
            .into_iter()
            .map(|(k,v)| (v,k))
            .collect::<BTreeMap<usize, String>>()
            .into_iter()
            .rev()
            .map(|(count, item)| {
                html!{ <> {format!("{item} ({count})")} <br/> </> }
            })
            .collect::<Vec<Html>>();

        let player_count = self.player_count();

        html! {
            <>
            <ybc::Navbar
                classes={classes!("is-primary")}
                padded=true
                navbrand={html!{
                    <ybc::NavbarItem>
                        <ybc::Title classes={classes!("has-text-white")} size={ybc::HeaderSize::Is4}>{"Battlebit Server Statistics"}</ybc::Title>
                    </ybc::NavbarItem>
                }}
                navburger=false
            />

            <ybc::Hero
                classes={classes!("is-dark")}
                size={ybc::HeroSize::FullheightWithNavbar}
                body={html!{    
                    <ybc::Tile ctx={Ancestor}>
                        <ybc::Tile ctx={Parent} size={ybc::TileSize::Twelve}>
                            <ybc::Tile ctx={Parent}>
                                <ybc::Tile ctx={Child} classes={classes!("notification", "is-primary")}>
                                    <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-white")}>{"Players"}</ybc::Subtitle>
                                    {format!("{} are playing, with another {} in the queue. ", player_count.0, player_count.1)}
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx={Parent}>
                                <ybc::Tile ctx={Child} classes={classes!("notification", "is-primary")}>
                                    <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-white")}>{"Maps"}</ybc::Subtitle>
                                    { maps }
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx={Parent}>
                                <ybc::Tile ctx={Child} classes={classes!("notification", "is-primary")}>
                                    <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-white")}>{"Regions"}</ybc::Subtitle>
                                    { regions }
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx={Parent}>
                                <ybc::Tile ctx={Child} classes={classes!("notification", "is-primary")}>
                                    <ybc::Subtitle size={ybc::HeaderSize::Is3} classes={classes!("has-text-white")}>{"Gamemodes"}</ybc::Subtitle>
                                    { gamemodes }
                                </ybc::Tile>
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Tile>
                }}
                foot={html!{
                    <ybc::Subtitle size={ybc::HeaderSize::Is6} classes={classes!("has-text-white")}>
                    {"Made with ♥︎ by "} <a href="https://github.com/superyu1337">{"Superyu1337"}</a>
                    </ybc::Subtitle>
                }}
                foot_classes={classes!("is-primary", "content", "has-text-centered")}
            />
            </>
        }
    }
}