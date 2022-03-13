fn tell_me_a_story(story: &str) {
    let a_good_story = story.to_owned() + "\nand lived happily ever after!";
    let _ = a_good_story.len();
}

pub fn main() {
    tell_me_a_story("A fox ran through the woods.");
}
