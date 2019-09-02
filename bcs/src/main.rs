//Blade of the Iron Throne Combat Simulator
use rand::Rng;
use std::io;
use std::process;
#[macro_use]
extern crate text_io;

fn main() {
    println!("Welcome to Blade of the Iron Throne Combat Simulator!");
    loop {
        println!("Press p(lay), i(nfo), q(uit).");
        let mut main_choice = String::new();
        io::stdin().read_line(&mut main_choice)
            .expect("Your command was not recognized. Tip: only single letters or numbers such as 'a', '2' or '24' are recognized.");

        struct CombatStats {
            pub brawn: u32,
            pub daring: u32,
            pub tenacity: u32,
            pub sagacity: u32,
            pub cunning: u32,
            pub reflex: u32,
            pub aim: u32,
            pub knockdown: u32,
            pub knockout: u32,
            pub movement: u32,
            pub mp: u32,
            pub brawl: u32,
            pub cut_n_thrust: u32,
            pub dagger: u32,
            pub greatsword: u32,
            pub lance: u32,
            pub longsword: u32,
            pub mass_n_shield: u32,
            //missile profs, not sure they should be distinct
            pub black_powder: u32,
            pub bow: u32,
            pub crossbow: u32,
            pub sling: u32,
            pub spear_javelin: u32,
            pub thrown_knife: u32,
            pub thrown_axe: u32,
            pub thrown_rock: u32,
            //
            pub pole_arms: u32,
            pub spear_n_shield: u32,
            pub sword_n_sheild: u32,
            pub wrestling: u32,
        }

        //consider making wounds structs and body parts initiations for said fields, but leaving the body part structs for other purposes.
        struct Loc {
            pub armor_lvl: u32,
            pub armor_type: u32,
            pub wound_type: u32,
            pub wound_lvl: u32,
        }

        struct combat {
            pub stance: u32,
        }
        struct Personal {
            pub name: String,
            pub sex: u32,
            pub pronoun: String,
            pub title: String,
            pub indef_article: String,
            pub def_article: String,
        }
        struct Health {
            pub pain: u32,
            pub shock: u32,
            pub fatigue: u32,
            pub blood_loss: u32,
        }

        //weapon struct

        let mut foot = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut shin_low_geg = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut knee_nearby = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut thigh = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut hip = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut up_abs = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut low_abs = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut ribcage = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut chest = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut neck = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut low_head = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut up_head = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut shoulder = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut in_thigh = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut groin = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut abdomen = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut hand = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut forearm = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut elbow = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };

        let mut en_foot = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_shin_low_geg = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_knee_nearby = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_thigh = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_hip = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_up_abs = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_low_abs = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_ribcage = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_chest = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_neck = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_low_head = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_up_head = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_shoulder = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_in_thigh = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_groin = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_abdomen = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_hand = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_forearm = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };
        let mut en_elbow = Loc {
            armor_lvl: 0,
            armor_type: 0,
            wound_type: 0,
            wound_lvl: 0,
        };

        let mut player_combat_stats = CombatStats {
            brawn: 0,
            daring: 0,
            tenacity: 0,
            sagacity: 0,
            cunning: 0,
            reflex: 0,
            aim: 0,
            knockdown: 0,
            knockout: 0,
            movement: 0,
            mp: 0,
            brawl: 0,
            cut_n_thrust: 0,
            dagger: 0,
            greatsword: 0,
            lance: 0,
            longsword: 0,
            mass_n_shield: 0,
            black_powder: 0,
            bow: 0,
            crossbow: 0,
            sling: 0,
            spear_javelin: 0,
            thrown_knife: 0,
            thrown_axe: 0,
            thrown_rock: 0,
            pole_arms: 0,
            spear_n_shield: 0,
            sword_n_sheild: 0,
            wrestling: 0,
        };

        let mut en_combat_stats = CombatStats {
            brawn: 0,
            daring: 0,
            tenacity: 0,
            sagacity: 0,
            cunning: 0,
            reflex: 0,
            aim: 0,
            knockdown: 0,
            knockout: 0,
            movement: 0,
            mp: 0,
            brawl: 0,
            cut_n_thrust: 0,
            dagger: 0,
            greatsword: 0,
            lance: 0,
            longsword: 0,
            mass_n_shield: 0,
            black_powder: 0,
            bow: 0,
            crossbow: 0,
            sling: 0,
            spear_javelin: 0,
            thrown_knife: 0,
            thrown_axe: 0,
            thrown_rock: 0,
            pole_arms: 0,
            spear_n_shield: 0,
            sword_n_sheild: 0,
            wrestling: 0,
        };

        let mut personal = Personal {
            sex: 1,
            name: String::new(),
            indef_article: "the".to_string(),
            def_article: "a".to_string(),
            title: "human".to_string(),
            pronoun: "he".to_string(),
        };
        let mut en_personal = Personal {
            sex: 1,
            title: "zombie".to_string(),
            indef_article: "a".to_string(),
            def_article: "the".to_string(),
            name: "Jim".to_string(),
            pronoun: "he".to_string(),
        };
        let mut game_con: u32 = 0;
        let mut char_con: u32 = 0;
        let mut experience: u32 = 35;
        const MAX_BL: u32 = 11;
        const MAX_SHOCK: u32 = 11;
        const MAX_PAIN: u32 = 11;
        const MAX_FATIGUE: u32 = 11;
        let mut exit: bool = false;

        let mut input_text = String::new();

        match main_choice.trim() {
            "p" => game_con = 1,
            "i" => println!("Here is some info about the game: "),
            "q" => exit = true,

            _ => {
                println!("Your command was not recognized.");
            }
        }

        if exit == true {
            println!("Fairwell.");
            break;
        }

        if game_con == 1 {
            //        let dice_roll = rand::thread_rng().gen_range(1, num_of_sides);
            println!("Do you want to make your own character? [y, n]");
            let mut char_choice = String::new();
            io::stdin()
                .read_line(&mut char_choice)
                .expect("Your command was not recognized.");
            match char_choice.trim() {
                "y" => char_con = 1,
                "n" => char_con = 2,
                //add exit menu option here
                _ => {
                    println!("Your command was not recognized.");
                }
            }
            if char_con == 1 {
                println!("What is your name?");
                io::stdin()
                    .read_line(&mut personal.name)
                    .expect("Your command was not recognized.");

                println!("What is your sex? [m, f]");
                let personal_sex: String = read!();
                if personal_sex == "m" {
                    personal.sex = 1;
                } else if personal_sex == "f" {
                    personal.sex = 2;
                }

                println!(
                    "You have 35 skill points.  Assign them to your 5 abiites and 19 proficencies."
                );
                println!("Abilities: ");
                println!("Brawn: [1-9]"); //v error handling
                loop {
                    let brawn_choice: u32 = read!();
                    if brawn_choice > 9 || brawn_choice < 1 {
                        println!("This number is not between 1 and 9!");
                    } else if brawn_choice == 1 {
                        player_combat_stats.brawn = 1;
                        experience -= brawn_choice;
                        break;
                    } else if brawn_choice < 9 || brawn_choice > 1 {
                        player_combat_stats.brawn = brawn_choice;
                        experience -= brawn_choice;
                        break;
                    }
                }
                println!("{} points remaining.", experience);
                println!("Daring: [1-9]");
                loop {
                    let daring_choice: u32 = read!();
                    if daring_choice > 9 || daring_choice < 1 {
                        println!("This number is not between 1 and 9!");
                    } else if daring_choice == 1 {
                        player_combat_stats.daring = 1;
                        experience -= daring_choice;
                        break;
                    } else if daring_choice < 9 || daring_choice > 1 {
                        player_combat_stats.daring = daring_choice;
                        experience -= daring_choice;
                        break;
                    }
                }
                println!("{} points remaining.", experience);
                println!("Tenacity: [1-9]");
                loop {
                    let tenacity_choice: u32 = read!();
                    if tenacity_choice > 9 || tenacity_choice < 1 {
                        println!("This number is not between 1 and 9!");
                    } else if tenacity_choice == 1 {
                        player_combat_stats.tenacity = 1;
                        experience -= tenacity_choice;
                        break;
                    } else if tenacity_choice < 9 || tenacity_choice > 1 {
                        player_combat_stats.tenacity = tenacity_choice;
                        experience -= tenacity_choice;
                        break;
                    }
                }
                println!("{} points remaining.", experience);
                println!("Sagacity: [1-9]");
                loop {
                    let sagacity_choice: u32 = read!();
                    if sagacity_choice > 9 || sagacity_choice < 1 {
                        println!("This number is not between 1 and 9!");
                    } else if sagacity_choice == 1 {
                        player_combat_stats.sagacity = 1;
                        experience -= sagacity_choice;
                        break;
                    } else if sagacity_choice < 9 || sagacity_choice > 1 {
                        player_combat_stats.sagacity = sagacity_choice;
                        experience -= sagacity_choice;
                        break;
                    }
                }
                println!("{} points remaining.", experience);
                println!("Cunning: [1-9]");
                loop {
                    let cunning_choice: u32 = read!();
                    if cunning_choice > 9 || cunning_choice < 1 {
                        println!("This number is not between 1 and 9!");
                    } else if cunning_choice == 1 {
                        player_combat_stats.cunning = 1;
                        experience -= cunning_choice;
                        break;
                    } else if cunning_choice < 9 || cunning_choice > 1 {
                        player_combat_stats.cunning = cunning_choice;
                        experience -= cunning_choice;
                        break;
                    }
                }
                println!("{} points remaining.", experience);
                println!("Choose your proficencies (each proficency costs one point): ");
                println!("[1(Brawling)]");
                println!("[2(Cut & Thrust)]");
                println!("[3(Dagger)]");
                println!("[4(Greatsword)]");
                println!("[5(Lance)]");
                println!("[6(Longsword)]");
                println!("[7(Mass Weapon & Shield)]");
                println!("[8(Black Powder Weapons)]");
                println!("[9(Bow)]");
                println!("[10(Crossbow)]");
                println!("[11(Sling)]");
                println!("[12(Spear or Javelin)]");
                println!("[13(Thrown Knife)]");
                println!("[14(Thrown Axe)]");
                println!("[15(Thrown Rock)]");
                println!("[16(Pole-arms)]");
                println!("[17(Spear & Shield)]");
                println!("[18(Sword & Shield)]");
                println!("[19(Wrestling)]");
                println!("[0(Next screen)]");

                loop {
                    let mut prof_choice: u32 = read!();
                    if prof_choice == 1 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point1_choice: u32 = read!();
                        if point1_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.brawl + point1_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point1_choice < experience || point1_choice == experience {
                            experience -= point1_choice;
                            player_combat_stats.brawl += point1_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 2 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point2_choice: u32 = read!();
                        if point2_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.cut_n_thrust + point2_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point2_choice < experience || point2_choice == experience {
                            experience -= point2_choice;
                            player_combat_stats.cut_n_thrust += point2_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 3 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point3_choice: u32 = read!();
                        if point3_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.dagger + point3_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point3_choice < experience || point3_choice == experience {
                            experience -= point3_choice;
                            player_combat_stats.dagger += point3_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 4 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point4_choice: u32 = read!();
                        if point4_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.greatsword + point4_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point4_choice < experience || point4_choice == experience {
                            experience -= point4_choice;
                            player_combat_stats.greatsword += point4_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 5 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point5_choice: u32 = read!();
                        if point5_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.lance + point5_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point5_choice < experience || point5_choice == experience {
                            experience -= point5_choice;
                            player_combat_stats.lance += point5_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 6 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point6_choice: u32 = read!();
                        if point6_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.longsword + point6_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point6_choice < experience || point6_choice == experience {
                            experience -= point6_choice;
                            player_combat_stats.longsword += point6_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 7 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point7_choice: u32 = read!();
                        if point7_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.mass_n_shield + point7_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point7_choice < experience || point7_choice == experience {
                            experience -= point7_choice;
                            player_combat_stats.mass_n_shield += point7_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 8 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point8_choice: u32 = read!();
                        if point8_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.black_powder + point8_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point8_choice < experience || point8_choice == experience {
                            experience -= point8_choice;
                            player_combat_stats.black_powder += point8_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 9 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point9_choice: u32 = read!();
                        if point9_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.bow + point9_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point9_choice < experience || point9_choice == experience {
                            experience -= point9_choice;
                            player_combat_stats.bow += point9_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 10 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point10_choice: u32 = read!();
                        if point10_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.crossbow + point10_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point10_choice < experience || point10_choice == experience {
                            experience -= point10_choice;
                            player_combat_stats.crossbow += point10_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 11 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point11_choice: u32 = read!();
                        if point11_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.sling + point11_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point11_choice < experience || point11_choice == experience {
                            experience -= point11_choice;
                            player_combat_stats.sling += point11_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 12 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point12_choice: u32 = read!();
                        if point12_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.spear_n_shield + point12_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point12_choice < experience || point12_choice == experience {
                            experience -= point12_choice;
                            player_combat_stats.spear_n_shield += point12_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 13 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point13_choice: u32 = read!();
                        if point13_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.thrown_knife + point13_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point13_choice < experience || point13_choice == experience {
                            experience -= point13_choice;
                            player_combat_stats.thrown_knife += point13_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 14 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point14_choice: u32 = read!();
                        if point14_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.thrown_axe + point14_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point14_choice < experience || point14_choice == experience {
                            experience -= point14_choice;
                            player_combat_stats.thrown_axe += point14_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 15 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point15_choice: u32 = read!();
                        if point15_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.thrown_rock + point15_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point15_choice < experience || point15_choice == experience {
                            experience -= point15_choice;
                            player_combat_stats.thrown_rock += point15_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 16 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point16_choice: u32 = read!();
                        if point16_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.pole_arms + point16_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point16_choice < experience || point16_choice == experience {
                            experience -= point16_choice;
                            player_combat_stats.pole_arms += point16_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 17 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point17_choice: u32 = read!();
                        if point17_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.spear_n_shield + point17_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point17_choice < experience || point17_choice == experience {
                            experience -= point17_choice;
                            player_combat_stats.spear_n_shield += point17_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 18 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point18_choice: u32 = read!();
                        if point18_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.sword_n_sheild + point18_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point18_choice < experience || point18_choice == experience {
                            experience -= point18_choice;
                            player_combat_stats.sword_n_sheild += point18_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 19 {
                        println!(
                            "How many points would you like to add to this proficiency? [1-9]"
                        );
                        let mut point19_choice: u32 = read!();
                        if point19_choice > experience {
                            println!("You don't have enough points left.");
                        } else if player_combat_stats.wrestling + point19_choice > 9 {
                            println!("You can only add up to 9 points to any one proficiency.");
                        } else if point19_choice < experience || point19_choice == experience {
                            experience -= point19_choice;
                            player_combat_stats.wrestling += point19_choice;
                            println!("{} points remain.", experience);
                        }
                    } else if prof_choice == 0 {
                        break;
                    }
                }
                loop {
                    println!("{} points remain.", experience);
                    println!("Select the part of the body on which you would like to add armor.");
                    println!("[1(Head and Neck)]");
                    println!("[2(Torso)]");
                    println!("[3(Shoulders and Biceps)]");
                    println!("[4(Forearms)]");
                    println!("[5(Hands)]");
                    println!("[6(Groin)]");
                    println!("[7(Thighs)]");
                    println!("[8(Knees)]");
                    println!("[9(Lower legs)]");
                    println!("[10(Feet)]");
                    println!("[0(Next Screen)]");
                    let mut armor_loc_choice: u32 = read!();
                    if armor_loc_choice == 1 {
                        println!("Select what type of armor to wear on this location.  Armor types with a '*' next to them automatically equip gambeson beneath.  Each type costs as much as the number left of it.");
                        println!("[1(Gambeson)]");
                        println!("[2(Leather)]");
                        println!("[3(Leather with Metal)]");
                        println!("[4(Hardened Leather)]");
                        println!("[5(Horn Scale*)]");
                        println!("[6(Lamellar*)]");
                        println!("[7(Brigandine*)]");
                        println!("[8(Maille*)]");
                        println!("[9(Brigandine over Maille*)]");
                        println!("[10(Plate*)]");
                        let mut armor_type_choice: u32 = read!();
                        up_head.armor_type = armor_type_choice;
                        low_head.armor_type = armor_type_choice;
                        neck.armor_type = armor_type_choice;
                        experience -= armor_type_choice;
                    }
                    if armor_loc_choice == 2 {
                        println!("Select what type of armor to wear on this location.  Armor types with a '*' next to them automatically equip gambeson beneath.  Each type costs as much");
                        println!("[1(Gambeson)]");
                        println!("[2(Leather)]");
                        println!("[3(Leather with Metal)]");
                        println!("[4(Hardened Leather)]");
                        println!("[5(Horn Scale*)]");
                        println!("[6(Lamellar*)]");
                        println!("[7(Brigandine*)]");
                        println!("[8(Maille*)]");
                        println!("[9(Brigandine over Maille*)]");
                        println!("[10(Plate*)]");
                        let mut armor_type_choice: u32 = read!();
                        up_head.armor_type = armor_type_choice;
                        low_head.armor_type = armor_type_choice;
                        neck.armor_type = armor_type_choice;
                        experience -= armor_type_choice;
                    }
                    if armor_loc_choice == 3 {
                        println!("Select what type of armor to wear on this location.  Armor types with a '*' next to them automatically equip gambeson beneath.  Each type costs as much");
                        println!("[1(Gambeson)]");
                        println!("[2(Leather)]");
                        println!("[3(Leather with Metal)]");
                        println!("[4(Hardened Leather)]");
                        println!("[5(Horn Scale*)]");
                        println!("[6(Lamellar*)]");
                        println!("[7(Brigandine*)]");
                        println!("[8(Maille*)]");
                        println!("[9(Brigandine over Maille*)]");
                        println!("[10(Plate*)]");
                        let mut armor_type_choice: u32 = read!();
                        up_head.armor_type = armor_type_choice;
                        low_head.armor_type = armor_type_choice;
                        neck.armor_type = armor_type_choice;
                        experience -= armor_type_choice;
                    }
                    if armor_loc_choice == 4 {
                        println!("Select what type of armor to wear on this location.  Armor types with a '*' next to them automatically equip gambeson beneath.  Each type costs as much");
                        println!("[1(Gambeson)]");
                        println!("[2(Leather)]");
                        println!("[3(Leather with Metal)]");
                        println!("[4(Hardened Leather)]");
                        println!("[5(Horn Scale*)]");
                        println!("[6(Lamellar*)]");
                        println!("[7(Brigandine*)]");
                        println!("[8(Maille*)]");
                        println!("[9(Brigandine over Maille*)]");
                        println!("[10(Plate*)]");
                        let mut armor_type_choice: u32 = read!();
                        up_head.armor_type = armor_type_choice;
                        low_head.armor_type = armor_type_choice;
                        neck.armor_type = armor_type_choice;
                        experience -= armor_type_choice;
                    }
                    if armor_loc_choice == 5 {
                        println!("Select what type of armor to wear on this location.  Armor types with a '*' next to them automatically equip gambeson beneath.  Each type costs as much");
                        println!("[1(Gambeson)]");
                        println!("[2(Leather)]");
                        println!("[3(Leather with Metal)]");
                        println!("[4(Hardened Leather)]");
                        println!("[5(Horn Scale*)]");
                        println!("[6(Lamellar*)]");
                        println!("[7(Brigandine*)]");
                        println!("[8(Maille*)]");
                        println!("[9(Brigandine over Maille*)]");
                        println!("[10(Plate*)]");
                        let mut armor_type_choice: u32 = read!();
                        up_head.armor_type = armor_type_choice;
                        low_head.armor_type = armor_type_choice;
                        neck.armor_type = armor_type_choice;
                        experience -= armor_type_choice;
                    }
                    if armor_loc_choice == 6 {
                        println!("Select what type of armor to wear on this location.  Armor types with a '*' next to them automatically equip gambeson beneath.  Each type costs as much");
                        println!("[1(Gambeson)]");
                        println!("[2(Leather)]");
                        println!("[3(Leather with Metal)]");
                        println!("[4(Hardened Leather)]");
                        println!("[5(Horn Scale*)]");
                        println!("[6(Lamellar*)]");
                        println!("[7(Brigandine*)]");
                        println!("[8(Maille*)]");
                        println!("[9(Brigandine over Maille*)]");
                        println!("[10(Plate*)]");
                        let mut armor_type_choice: u32 = read!();
                        up_head.armor_type = armor_type_choice;
                        low_head.armor_type = armor_type_choice;
                        neck.armor_type = armor_type_choice;
                        experience -= armor_type_choice;
                    }
                    if armor_loc_choice == 7 {
                        println!("Select what type of armor to wear on this location.  Armor types with a '*' next to them automatically equip gambeson beneath.  Each type costs as much");
                        println!("[1(Gambeson)]");
                        println!("[2(Leather)]");
                        println!("[3(Leather with Metal)]");
                        println!("[4(Hardened Leather)]");
                        println!("[5(Horn Scale*)]");
                        println!("[6(Lamellar*)]");
                        println!("[7(Brigandine*)]");
                        println!("[8(Maille*)]");
                        println!("[9(Brigandine over Maille*)]");
                        println!("[10(Plate*)]");
                        let mut armor_type_choice: u32 = read!();
                        up_head.armor_type = armor_type_choice;
                        low_head.armor_type = armor_type_choice;
                        neck.armor_type = armor_type_choice;
                        experience -= armor_type_choice;
                    }
                    if armor_loc_choice == 8 {
                        println!("Select what type of armor to wear on this location.  Armor types with a '*' next to them automatically equip gambeson beneath.  Each type costs as much");
                        println!("[1(Gambeson)]");
                        println!("[2(Leather)]");
                        println!("[3(Leather with Metal)]");
                        println!("[4(Hardened Leather)]");
                        println!("[5(Horn Scale*)]");
                        println!("[6(Lamellar*)]");
                        println!("[7(Brigandine*)]");
                        println!("[8(Maille*)]");
                        println!("[9(Brigandine over Maille*)]");
                        println!("[10(Plate*)]");
                        let mut armor_type_choice: u32 = read!();
                        up_head.armor_type = armor_type_choice;
                        low_head.armor_type = armor_type_choice;
                        neck.armor_type = armor_type_choice;
                        experience -= armor_type_choice;
                    }
                    if armor_loc_choice == 9 {
                        println!("Select what type of armor to wear on this location.  Armor types with a '*' next to them automatically equip gambeson beneath.  Each type costs as much");
                        println!("[1(Gambeson)]");
                        println!("[2(Leather)]");
                        println!("[3(Leather with Metal)]");
                        println!("[4(Hardened Leather)]");
                        println!("[5(Horn Scale*)]");
                        println!("[6(Lamellar*)]");
                        println!("[7(Brigandine*)]");
                        println!("[8(Maille*)]");
                        println!("[9(Brigandine over Maille*)]");
                        println!("[10(Plate*)]");
                        let mut armor_type_choice: u32 = read!();
                        up_head.armor_type = armor_type_choice;
                        low_head.armor_type = armor_type_choice;
                        neck.armor_type = armor_type_choice;
                        experience -= armor_type_choice;
                    }
                    if armor_loc_choice == 10 {
                        println!("Select what type of armor to wear on this location.  Armor types with a '*' next to them automatically equip gambeson beneath.  Each type costs as much");
                        println!("[1(Gambeson)]");
                        println!("[2(Leather)]");
                        println!("[3(Leather with Metal)]");
                        println!("[4(Hardened Leather)]");
                        println!("[5(Horn Scale*)]");
                        println!("[6(Lamellar*)]");
                        println!("[7(Brigandine*)]");
                        println!("[8(Maille*)]");
                        println!("[9(Brigandine over Maille*)]");
                        println!("[10(Plate*)]");
                        let mut armor_type_choice: u32 = read!();
                        up_head.armor_type = armor_type_choice;
                        low_head.armor_type = armor_type_choice;
                        neck.armor_type = armor_type_choice;
                        experience -= armor_type_choice;
                    }
                    if armor_loc_choice == 0 {
                        println!("{} points remain.", experience);
                        break;
                    }
                }
                println!("Select your starting weapons.");
                println!("[1(Greatsword)]");
                println!("[2(Leather)]");
                println!("[3(Leather with Metal)]");
                println!("[4(Hardened Leather)]");
                println!("[5(Horn Scale*)]");
                println!("[6(Lamellar*)]");
                println!("[7(Brigandine*)]");
                println!("[8(Maille*)]");
                println!("[9(Brigandine over Maille*)]");
                println!("[10(Plate*)]");
            }
        } else if char_con == 2 {
            let mut personal = Personal {
                name: "James".to_string(),
                indef_article: "a".to_string(),
                title: "human".to_string(),
                sex: 1,
                pronoun: "he".to_string(),
                def_article: "the".to_string(),
            };
        }
        println!("Get ready to fight!"); //for now all combat will be gladitorial bouts in an abstract setting
        println!(
            "Before you is {} {} {}, {} wears armor and wields weapon.",
            en_personal.name, en_personal.def_article, en_personal.title, en_personal.pronoun,
        );
        /*        println!("Declare your stance: a(ggresive), n(eutral), d(efensive).");
        let mut stance = String::new();
        io::stdin().read_line(&mut stance)
        .expect("Your command was not recognized. Tip: only single letters or numbers such as 'a', '2' or '24' are recognized.");
            match main_choice.trim() {
            "a" => combat.stance = 1,
            "n" => combat.stance = 2,
            "d" => combat.stance = 3,
               _   => {
              println!("Your command was not recognized. Tip: only single letters or numbers such as 'a', '2' or '24' are recognized.");
              }
            } */
    }
}
