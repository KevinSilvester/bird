use super::command::Command;
use crate::core::BirdConfig;
use crate::utils::colour;
use crate::utils::errors::BirdError;
// use anyhow::Result;

#[derive(clap::Parser, Debug)]
pub struct Chirp;

impl Command for Chirp {
   fn call(self, config: &BirdConfig) -> Result<(), BirdError> {
      let ascii = r#"                                                                                                          ^~.                                           
                                                                                                        !. !J7.                                         
  _____  _                     ____   _           _                                                     7J!^?J?^?                                       
 |_   _|( )                   |  _ \ (_)         | |                                               .^!7?JJJJJJJJJ7!~:.                                  
   | |  |/ _ __ ___     __ _  | |_) | _  _ __  __| |                                           .^7JJJJJJJJJJJJJJJJJJJJ?7^                               
   | |    | '_ ` _ \   / _` | |  _ < | || '__|/ _` |                                         .!JJJJJJJJJJJJJJJJJJJJJJJJJJJ!.                            
  _| |_   | | | | | | | (_| | | |_) || || |  | (_| |                                       .!JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ~                           
 |_____|  |_| |_| |_|  \__,_| |____/ |_||_|   \__,_|                                      .?JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ7                          
   _____  _      _               _                                                       .JJJJJJJJJJJJJJJJJJJJJJ??J?JJJJJJJJJJ?                         
  / ____|| |    (_)             | |                                                     .?JJJJYJJJJJJJJJJJJJYYY?55J?YJJJJJJJJJJ7                        
 | |     | |__   _  _ __  _ __  | |                                                     ^JY5J?Y?YJJJJJJJJYYY55Y7?J?YYJJJJJJJJJJJ:                       
 | |     | '_ \ | || '__|| '_ \ | |                                                   :!?JJJJ?77!~^!JJJJJJJJJJJJJJYJJJJJJJJJJJJJ7                       
 | |____ | | | || || |   | |_) ||_|                                                 .?JJJJJJ?!^:::::7JJJJJJJJJJJJJJJJJJJJJJJJJJJ?                       
  \_____||_| |_||_||_|   | .__/ (_)                                                 7JJJJJJJJJ?!~~~~?JJJJJJJJJJJJJJJJJJJJJJJJJJJJ.                      
   _____  _      _       | |     _                           .     .     .          !JJJJJJJJJJJJ???JJJJJJJJJJJJJJJJJJJJJJJJJJJJJ^                      
  / ____|| |    (_)      |_|    | |                         .     .     .           .?JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ!                      
 | |     | |__   _  _ __  _ __  | |                          .     .     .           :JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ?                      
 | |     | '_ \ | || '__|| '_ \ | |                                                  ^JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ.                     
 | |____ | | | || || |   | |_) ||_|                      ~#5#@@@@@@@@@@@@@@@@@~      ~JJJJYJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ~                     
  \_____||_| |_||_||_|   | .__/ (_)                       5!P@&&&&&&&&&&&&&&&@^      ~JJJJJYYYYJ??JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ?                     
                         | |                              7PG@@@@@@@@@@@@@@@@#Y:^~^ .7JJJJJJJYJ??JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ.                    
                         |_|                              .BG@@@&&@&&&&@@&@@@YYYJJYJJYJJJJJJ557??JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ~                    
                                                           5G&@@@@@@@@@@@@@@&?YYJJYYJYJJJJJJJ5?7J??JJJJJJJJJJJJYJJJJJJJJJJJJJJYJJJJ?                    
                                                           ^B#@@@@@@@@@@@@@@P?Y5YJJJJ5JJJJJJJJYYYJJJJJJJJJJJJJ?JJJJJJJJJJJJJJJJYJJJJ^                   
                                                            GG@@@@@@@@@@@@@@PY55YJJJJPJJJJJJJJJJJJJJJJJJJJJJJ???J?JJJJJJJJJJJJJJ5JJJ?                   
                                                            ?B&@@@@@@@@@@@@@#P5YYJJ???7JJ?JJJJJJJJJJJJJJ?J???J??JJ7?JJJ?JJJJJJJJJ5YJJ~                  
                                                            .##@@@@@@@@@@@@7!!!~^:..  .J???J??????????J??J???J???JJ77?J??JJJJJJJJJYYJJ.                 
                                                             7B@@@@@@@@@@@?            7JJJJJJJJJJJJJJJJJJJJJJJJJJJYJ?7?JJJJJ?JJJJ?J5Y?.                
                                                               ::::::::::              :JJJJJJJJJJJJJJJJJJJJJJJJJJJJJYJJ?????JJJJ5YJJJJ?.               
                                                                                        !JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ?J5JJJJJJJJJJ!:.....^!.     
                                                                                         ?JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ!      
                                                                                         .?JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ7       
                                                                                          .?JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ7        
                                                                                           .7JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ!         
                                                                                             ^JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ7:          
                                                                                              .!JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ7:            
                                                                                                .~?JJJJJJJJJJJJJJJJYYJJJJJJJJJJJJJJJJJJ?~.              
                                                                                                   .~7JJJJJJJJJJJJJJJY?7JJJJJJJJJJJ?!^.                 
                                                                                                      :7!!7??JJJJJJJJJ~^JJJ??7!~^:.                     
                                                                                                      .^.     ........:^..                              
                                                                                                      .^.             .^                                
                                                                                                      .^.             .^                                
                                                                     ...::^^~~!!777???JJJJYYYYYY5YY?!^^~5555YYYYYYYYJ~^7J??777!!~~^^::...              
                                                              :7J5PGGGBBBBBBBBBBBBBBBBBBBBBBBBBBG5J?7777?GBBBBBBBG5J7~^^7BBBBBBBBBBBBBBBBBGGPP5J7.      
                                                              .^~7?JY5PPGGGGBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBGPPPPGGGGBBBBBBBBBBBGGGGPP5YJ?7~^.      
                                                                          .....:::^^^~~~~!!!!!777777777777777777777!!!!~~~~^^^:::....."#;
      println!("{}", colour::info(ascii));
      Ok(())
   }
}
