pub mod configuration;
pub mod donate;
pub mod info;
pub mod invite;
pub mod silence;
pub mod start;
pub mod stats;
pub mod time;

use configuration::*;
use donate::*;
use info::*;
use invite::*;
use silence::*;
use start::*;
use stats::*;
use time::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(configuration, donate, info, invite, silence, start, stats, time)]
struct General;
