use rise_of_industry_caculator::simulator::*;


fn main() {
    recipe::RECIPES.iter().for_each(|(name, recipe)| {
        println!("name: {}, recipe: {:?}", name, recipe);
    });
    building::INFOS.iter().for_each(|(b_type, inf)| {
        println!("type: {:?}, info: {:?}", b_type, inf);
    });
}
