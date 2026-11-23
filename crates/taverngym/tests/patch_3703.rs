//! 37.0.3 server hotfix (2026-11-17): Snarky Shark, Felboar and Sanguine Champion, Trinket costs,
//! Emergency Gearblade, Kith'ix and the late Dark Gift Discover rule.
mod common;
use common::*;
use taverngym::content;

fn lobby(seed: u64, types: &[&str]) -> Game {
    Game::new(
        seed,
        RulesConfig {
            lobby_races: types.len(),
            lobby_types: types.iter().map(|t| t.to_string()).collect(),
            aberration_forced: false,
            ..RulesConfig::default()
        },
    )
}

#[test]
fn snarky_shark_is_tier_5() {
    for id in ["BG36_206", "BG36_206_G"] {
        assert_eq!(db().get(cid(id)).tier, 5, "{id}");
    }
}

#[test]
fn sanguine_champion_gives_blood_gems_one_one() {
    // "Battlecry and Deathrattle: Your Blood Gems give an extra +1/+1 this game." (+2/+1 before;
    // Golden +2/+2.)
    for (card, k) in [("BG23_017", 1), ("BG23_017_G", 2)] {
        let mut g = game(370301);
        let before = (g.players[0].stats.gem_atk, g.players[0].stats.gem_hp);
        give(&mut g, 0, card);
        play_last(&mut g, 0);
        let st = &g.players[0].stats;
        assert_eq!(
            (st.gem_atk, st.gem_hp),
            (before.0 + k, before.1 + k),
            "{card}"
        );
    }
}

#[test]
fn hotfix_trinket_costs() {
    for (id, cost) in [
        ("BG30_MagicItem_962", 5),  // Training Certificate
        ("BG32_MagicItem_808", 3),  // Bloodbound Earrings
        ("BG32_MagicItem_892", 4),  // Vibrant Bubble
        ("BG36_MagicItem_409", 1),  // Ruby Tusk
        ("BG35_MagicItem_306", 3),  // Jailer Sticker
        ("BG30_MagicItem_411", 0),  // Hoggy Bank
        ("BG36_MagicItem_307", 1),  // Wand of Divination
        ("BG32_MagicItem_278", 0),  // Reusable Batteries
        ("BG36_MagicItem_390", 0),  // Baller Portrait (Lesser)
        ("BG30_MagicItem_821", 0),  // Fish Portrait
        ("BG36_MagicItem_800", 1),  // Demonic Bloodletter
        ("BG35_MagicItem_150", 0),  // Cursed Crystal
        ("BG36_MagicItem_402", 2),  // Mindslasher Portrait
        ("BG30_MagicItem_442", 7),  // Blood Golem Sticker
        ("BG36_MagicItem_450", 5),  // Snarky Portrait
        ("BG35_MagicItem_732", 7),  // Soul Fermenter
        ("BG36_MagicItem_308", 6),  // Trailblazer Sticker
        ("BG36_MagicItem_414", 5),  // Weighted Gauntlet
        ("BG36_MagicItem_423t", 0), // Volumizer Portrait (Greater)
        ("BG32_MagicItem_361t", 1), // Portable Factory (Greater)
        ("BG30_MagicItem_801", 1),  // Sinstone Sticker
        ("BG32_MagicItem_284", 0),  // Aggem Sticker
        ("BG30_MagicItem_914t", 0), // Nerglish Phrasebook (Greater)
        ("BG35_MagicItem_740", 0),  // Sky Golem Portrait
    ] {
        assert_eq!(db().get(cid(id)).cost, cost, "{id}");
    }
}

#[test]
fn emergency_gearblade_left_the_lesser_trinket_pool() {
    let gearblade = cid("BG36_MagicItem_812");
    let bubble = cid("BG32_MagicItem_892"); // a Lesser Trinket that stays (no type)
    for seed in 0..20 {
        let g = lobby(seed, &["MECHANICAL", "BEAST", "DEMON", "MURLOC", "PIRATE"]);
        assert!(!g.trinket_universe.contains(&gearblade), "seed {seed}");
        assert!(g.trinket_universe.contains(&bubble), "seed {seed}");
    }
}

fn offered(g: &Game, hero: &str) -> bool {
    g.players.iter().any(|p| {
        p.pending
            .iter()
            .any(|x| matches!(x, Pending::HeroSelect { options } if options.contains(&cid(hero))))
    })
}

#[test]
fn kithix_is_offered_only_with_aberrations() {
    const KITHIX: &str = "BG36_HERO_002";
    for seed in 0..20 {
        // Undead (its type) without Aberrations: never; with both: guaranteed as before.
        let g = lobby(seed, &["UNDEAD", "BEAST", "DEMON", "MURLOC", "PIRATE"]);
        assert!(!offered(&g, KITHIX), "seed {seed}");
        let g = lobby(seed, &["UNDEAD", "ABERRATION", "DEMON", "MURLOC", "PIRATE"]);
        assert!(offered(&g, KITHIX), "seed {seed}");
    }
}

#[test]
fn late_dark_gift_discovers_need_the_most_common_type() {
    let mut g = game(370302); // Aberrations are in this lobby.
    let lobby = g.lobby_races;
    assert!(lobby.intersects(Races::ABERRATION));
    let ok = |g: &mut Game, id: &str| {
        let card = db().get(cid(id));
        g.with_recruit(0, |c| content::late_gift_ok(c, 0, card))
    };
    // Two Aberrations and a Mech in the warband.
    set_board(&mut g, 0, &["BG36_102", "BG36_102", "BG35_342"]);
    for turn in [8, 9] {
        g.turn = turn;
        for id in content::LATE_TYPE_ONLY {
            assert!(ok(&mut g, id), "{id} turn {turn}");
        }
    }
    g.turn = 10;
    assert!(
        ok(&mut g, "BG36_102"),
        "De-volition-ist matches Aberrations"
    );
    assert!(
        ok(&mut g, "BG36_102_G"),
        "Golden forms follow the normal one"
    );
    assert!(
        !ok(&mut g, "BG35_342"),
        "Falling Sky Golem: Mech is not the most common"
    );
    assert!(!ok(&mut g, "BG36_356"), "Tyrael has no type");
    assert!(!ok(&mut g, "BG35_123"), "Cataclysmic Harbinger has no type");
    // Other minions are not restricted.
    assert!(ok(&mut g, "BG36_345"));
    // A tie allows each tied type.
    set_board(&mut g, 0, &["BG36_102", "BG35_342"]);
    assert!(ok(&mut g, "BG36_102") && ok(&mut g, "BG35_342"));
    // An empty warband has no most common type.
    set_board(&mut g, 0, &[]);
    assert!(!ok(&mut g, "BG35_342"));
}

#[test]
fn late_dark_gift_button_skips_restricted_minions() {
    let restricted: Vec<CardId> = content::LATE_TYPE_ONLY.iter().map(|x| cid(x)).collect();
    for seed in 0..40 {
        let mut g = game(370400 + seed);
        g.turn = 11;
        g.players[0].tier = 6;
        // No Mech or Aberration on board: none of the four fits.
        set_board(&mut g, 0, &["BG36_345"]);
        let options = g.with_recruit(0, |c| c.dark_gift_options(0));
        for o in &options {
            let Item::Minion(m) = o else { continue };
            assert!(
                !restricted.contains(&db().normal_of(m.card)),
                "seed {seed}: {}",
                db().get(m.card).name
            );
        }
    }
}
