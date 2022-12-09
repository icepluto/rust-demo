fn main(){
    let a_tweet = Tweet{
        username:String::from("kitty"),
        content:String::from("it is a good day"),
        reply:false,
        reweet:false,
    };
    println!("{}",a_tweet.summarize());
}
pub trait Summary {
    fn summarize(self)->String;
}
struct NewsArticles{
    headline:String,
    location:String,
    content:String,
    author:String,
}
impl Summary for NewsArticles{
    fn summarize(self)->String{
        format!("{}{}{},发表于:{}",self.author,self.headline,self.content,self.location)
    }
}
struct Tweet{
    username:String,
    content:String,
    reply:bool,
    reweet:bool,
}
impl Summary for Tweet {
    fn summarize(self)->String {
        format!("username:{},content: {}, reply:{},reweet:{}",self.username,self.content,self.reply,self.reweet)
    }
}