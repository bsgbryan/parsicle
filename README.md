# Overview

Hello there, thanks for stopping by 😊

`parsicle` is a little library for organizing web content - specifically articles and article-like content.

The core of `parsicle` is this:

```rust
use chrono::{
  DateTime,
  Utc,
};
use url::Url;

pub struct Author {
  pub href: Url,
  pub name: String,
}

pub struct Image {
  pub href:    Url,
  pub caption: String,
  pub credit:  String,
}

pub struct Article {
  pub alternate:   Option<Vec<(String, Url)>>,
  pub authors:     Vec<Author>,
  pub canonical:   Url,
  pub content:     Option<Vec<String>>,
  pub description: Option<String>,
  pub hero_image:  Option<Url>,
  pub images:      Option<Vec<Image>>,
  pub published:   Option<DateTime<Utc>>,
  pub title:       String,
}
```

The [sources directory](./src/sources/) lists everywhere `parsicle` knows how to process content from.

## Usage

```rust
use parsicle::parse::parse;

#[tokio::main]
async fn main() {
  parse("https://www.cnn.com/2025/06/11/politics/gavin-newsom-democrats-fight-trump").await.ok().unwrap();
}
```

This will result in the following:

```
ARTICLE: Newsom and California confront Trump with a potential blueprint for Democrats
  published: 2025-06-11 10:00:03.539 UTC
  hero image: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2210604711.jpg
  canonical url: https://www.cnn.com/2025/06/11/politics/gavin-newsom-democrats-fight-trump
  alternate url -> lang: en-gb, href: https://edition.cnn.com/2025/06/11/politics/gavin-newsom-democrats-fight-trump
  alternate url -> lang: en-ca, href: https://www.cnn.com/2025/06/11/politics/gavin-newsom-democrats-fight-trump
  alternate url -> lang: en-us, href: https://www.cnn.com/2025/06/11/politics/gavin-newsom-democrats-fight-trump
  alternate url -> lang: x-default, href: https://edition.cnn.com/2025/06/11/politics/gavin-newsom-democrats-fight-trump
  description: Democratic politicians have spent the last few months talking about standing up to President Donald Trump in his second term. California Gov. Gavin Newsom is among the first faced with figuring out what standing up actually looks like.
AUTHOR -> Edward-Isaac Dovere: https://www.cnn.com/profiles/isaac-dovere
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2219193186.jpg
  credit: Gina Ferazzi/Los Angeles Times/Getty Images
  caption: Pictures of people arrested by ICE hang on a sculpture in Little Tokyo in Los Angeles, on Thursday, June 12.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2219695699.jpg
  credit: Mario Tama/Getty Images
  caption: Maricela Martinez of the mariachi band Mariachi Lindas Mexicanas performs during a protest by mariachi and folklorico dancers outside City Hall in Los Angeles on Wednesday, as police officers keep watch.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/2025-06-12t014725z-205847203-rc2p0fad005a-rtrmadp-3-usa-migration-protest-los-angeles.jpg
  credit: David Ryder/Reuters
  caption: Mounted police officers attempt to disperse protesters in downtown Los Angeles on Wednesday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/ap25163200882457.jpg
  credit: Ethan Swope/AP
  caption: A man shouts into a megaphone outside City Hall on Wednesday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/ap25163015789201.jpg
  credit: Jae C. Hong/AP
  caption: Law enforcement blocks a road during a protest in Paramount, California, on Wednesday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/2025-06-12t025949z-676067069-rc2q0fa13zmz-rtrmadp-3-usa-migration-protest-los-angeles.jpg
  credit: David Swanson/Reuters
  caption: A law enforcement officer shoots a non-lethal weapon at a protester on Wednesday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2219574817-20250611090210116.jpg
  credit: Mario Tama/Getty Images
  caption: Police move to enforce an 8 p.m. curfew in downtown Los Angeles, after it went into effect on Tuesday, June 10. Mayor Karen Bass <a href="https://www.cnn.com/us/live-news/la-protests-ice-raids-trump-06-11-25#cmbravjug001w26pa23t08ryw">declared a curfew for the 1 square mile</a> in which protests have been centered.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/ap25162028555942.jpg
  credit: Eric Thayer/AP
  caption: A protester is arrested by California Highway Patrol near the federal building in downtown Los Angeles on Tuesday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2219575191.jpg
  credit: Spencer Platt/Getty Images
  caption: Protesters move through downtown Los Angeles on Tuesday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218825947.jpg
  credit: Nick Ut/Getty Images
  caption: Protesters clash with police on the 101 Freeway in Los Angeles on Tuesday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218965901.jpg
  credit: Robyn Beck/AFP/Getty Images
  caption: Members of the clergy and other protestors place flowers at the feet of a California National Guardsman stationed outside federal buildings near the Metropolitan Detention Center in Los Angeles on Tuesday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/2025-06-10t140029z-856991549-rc2qzeain0s8-rtrmadp-3-usa-migration-protest-los-angeles.jpg
  credit: David Ryder/Reuters
  caption: Workers board up a CVS Pharmacy on Tuesday after it was looted following days of protests.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2219421581-20250610084224033.jpg
  credit: Spencer Platt/Getty Images
  caption: Police engage with protesters on Monday, June 9.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/2025-06-10t050337z-344932719-rc2fzeafwf77-rtrmadp-3-usa-migration-protest-los-angeles.JPG
  credit: Aude Guerrucci/Reuters
  caption: Demonstrators wave Mexican flags in downtown Los Angeles while protesting federal immigration sweeps on Monday.
IMAGE                                                                                                                                                                                         href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2219404489.jpg
  credit: Spencer Platt/Getty Images
  caption: People protest in Los Angeles on Monday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218871599.jpg
  credit: Joshua Lott/The Washington Post/Getty Images
  caption: Protesters are rounded up by police officers outside the Metropolitan Detention Center in Los Angeles on Monday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218877876.jpg
  credit: Jim Vondruska/Getty Images
  caption: A police helicopter <a href="https://www.cnn.com/us/live-news/la-protests-ice-raids-trump-06-10-25#cmbq6ad6p00003b6mzio8tnmo">hovers over protesters</a> outside of the Robert A. Young Federal Building on Monday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gall01-2025-06-10t013424z-601196579-rc28zea2llr3-rtrmadp-3-usa-migration-protest-los-angeles-jpg.jpg
  credit: Leah Millis/Reuters
  caption: A police officer fires a crowd control munition into the ground while confronting demonstrators on Monday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gall02-2025-06-09t231826z-747344156-rc2bzea3lct2-rtrmadp-3-usa-migration-protest-los-angeles-jpg.jpg
  credit: Daniel Cole/Reuters
  caption: Protesters are reflected in the sunglasses of a law enforcement officer on Monday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gall03-gettyimages-2219396833-jpg.jpg
  credit: Mario Tama/Getty Images
  caption: People gather during a protest outside a federal building on Monday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gall04-2025-06-09t221138z-1815504328-rc2azeajb754-rtrmadp-3-usa-migration-protest-los-angeles-jpg.jpg
  credit: Daniel Cole/Reuters
  caption: California labor leader <a href="https://www.cnn.com/2025/06/10/us/david-huerta-seiu-union-leader">David Huerta</a> speaks to the media after his release from detention in downtown Los Angeles on Monday. Huerta was charged with one count of conspiracy to impede an officer after he blocked access to a gate during an immigration protest, according to prosecutors. He was released from custody on $50,000 bond, the US Attorney’s office for the Central District of California said. Speaking outside the courthouse after his release, Huerta said authorities are trying to make an example out of him.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/ap25160012088901.jpg
  credit: Jae C. Hong/AP
  caption: Protesters confront police on the 101 Freeway near the Metropolitan Detention Center on Sunday, June 8.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/2025-06-09t042212z-296507235-rc2syea5gy5d-rtrmadp-3-usa-migration-protest-los-angeles.jpg
  credit: Aude Guerrucci/Reuters
  caption: A demonstrator waves a Mexican flag in front of burning dumpsters in downtown Los Angeles on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218766019.jpg
  credit: Jim Vondruska/Getty Images
  caption: Protesters stand in front of police blocking a bridge over the 101 Freeway on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/ap25160028602910.jpg
  credit: Jae C. Hong/AP
  caption: Two Waymo taxis burn near the Metropolitan Detention Center on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/ap25160100698446.jpg
  credit: Jae C. Hong/AP
  caption: Two California Highway Patrol officers try to dodge rocks being thrown near the Metropolitan Detention Center on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/2025-06-09t025202z-2135746734-rc2lyeags6tz-rtrmadp-3-usa-migration-protest-los-angeles.JPG
  credit: Barbara Davidson/Reuters
  caption: Officers detain a demonstrator on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/2025-06-09t101257z-1472830918-rc2ryeamohp0-rtrmadp-3-usa-migration-protest-los-angeles.JPG
  credit: David Ryder/Reuters
  caption: Police clear demonstrators after they blocked a street with a barricade on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218766048.jpg
  credit: Jim Vondruska/Getty Images
  caption: Protesters hide behind barricades as law enforcement officers shoot projectiles at them on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218772828.jpg
  credit: David Pashaee/Middle East Images/AFP/Getty Images
  caption: Smoke fills the air as law enforcement officers in riot gear advance during protests on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/2025-06-09t013242z-784388850-rc2lyea8ddyp-rtrmadp-3-usa-migration-protest-los-angeles.JPG
  credit: Barbara Davidson/Reuters
  caption: A police officer treats an injury on his face on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218708329.jpg
  credit: Etienne Laurent/AFP/Getty Images
  caption: Law enforcement officers clash with demonstrators on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2219241562.jpg
  credit: Spencer Platt/Getty Images
  caption: Police officers on horseback clash with protesters on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/2025-06-08t144406z-1295295523-rc2wxea9ev95-rtrmadp-3-usa-migration-protest-los-angeles.JPG
  credit: Barbara Davidson/Reuters
  caption: Los Angeles County sheriff's deputies detain a woman on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218720080.jpg
  credit: Robert Gauthier/Los Angeles Times/Getty Images
  caption: A protester damages a self-driving Waymo taxi on Sunday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218511571.jpg
  credit: Ringo Chiu/AFP/Getty Images
  caption: Demonstrators stand on top of a charred vehicle during a protest in the Compton neighborhood of Los Angeles.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/2025-06-08t144232z-1402147423-rc2zxeath0gu-rtrmadp-3-usa-migration-protest-los-angeles.JPG
  credit: Barbara Davidson/Reuters
  caption: A man on a motorcycle waves a Mexican flag as smoke rises from a burning car on Saturday, June 7.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218511484.jpg
  credit: Gina Ferazzi/Los Angeles Times/Getty Images
  caption: Protesters shield themselves against law enforcement during a protest on Saturday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218526283.jpg
  credit: Ringo Chiu/AFP/Getty Images
  caption: Los Angeles County sheriff's deputies clash with demonstrators on Saturday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218523462-1.jpg
  credit: Ringo Ciu/AFP/Getty Images
  caption: A protester reacts as law enforcement officers clash with demonstrators on Saturday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218511481.jpg
  credit: Ringo Chiu/AFP/Getty Images
  caption: Law enforcement officers face protesters on Saturday.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218402277.jpg
  credit: Jason Armond/Los Angeles Times/Getty Images
  caption: Police clear a street outside the Metropolitan Detention Center as demonstrators gathered in response to immigration raids on Friday, June 6.
IMAGE
  href: https://media.cnn.com/api/v1/images/stellar/prod/gettyimages-2218304113.jpg
  credit: Genaro Molina/Los Angeles Times/Getty Images
  caption: People gather in front of Ambiance Apparel in the garment district of downtown Los Angeles after several employees were taken into custody by federal agents on Thursday, June 5.
CONTENT:
    Democratic politicians have spent the last few months talking about standing up to President Donald Trump in his second term. California Gov. Gavin Newsom is among the first faced with figuring out what standing up actually looks like.
  Allies and opponents agree how Newsom handles the protests – including Trump’s calling in the National Guard and sending in active-duty Marines over the governor’s objections – will reverberate far beyond California, and long after this week.
  That’s how Newsom is approaching what has become a fight on the streets and in the courts, only a few days after he was responding to a Trump administration effort to identify federal grants going to the state that can be canceled.
  Other Democratic governors have been calling Newsom, checking in, ticking through scenarios in their minds of how what’s happened in California could play out at home for them, according to multiple people briefed on the conversations.
  Every Democratic governor signed onto a statement over the weekend calling Trump’s call-up of the National Guard an “alarming abuse of power,” but they have been treading carefully since then, their eyes on both the politics of potentially triggering Trump and on the legal concerns of how their words might be used in lawsuits they might have to bring.
  Newsom, people familiar with his thinking say, wants California to hold the line after some universities and law firms facing White House pressure reached concession deals with the administration.
  “What Donald Trump wants most is your fealty. Your silence. To be complicit in this moment,” Newsom said in remarks released Tuesday evening. “Do not give into him.”
  “If some of us can be snatched off the streets without a warrant – based only on suspicion or skin color – then none of us are safe. Authoritarian regimes begin by targeting people who are least able to defend themselves. But they do not stop there,” Newsom said, reiterating accusations that Trump officials instigated and inflamed what started as peaceful protests, though there have been skirmishes and occasional violence that Newsom and others have condemned.
  “This is about all of us. This is about you,” he said. “California may be first – but it clearly won’t end here. Other states are next. Democracy is next.”
  === An issue for this year and for 2028 ===
  As obvious as Newsom’s presidential ambitions are, several top Democrats say this is much more about America over the next few months than any talk of the 2028 presidential primary.
  Connecticut Sen. Chris Murphy, another potential 2028 candidate, has become one of the most outspoken Democrats calling attention to what he says is Trump’s direct threat to democracy in his second term.
  Trump “is clearly trying to scare his opposition into silence, and that is definitely one of the ways that democracies die: when people fear that they are going to face physical harm if they turn out for protests, it often causes people to stay home. That is a tried and true path for democracies to be converted into autocracies. Elections still happen, but the opposition can never amount to any kind of numbers because people fear they’ll get the shit kicked out of them if they show up,” Murphy told CNN.
  New Jersey Sen. Cory Booker, who acknowledged his record-breaking 25-hour Senate speech came during a different phase both for Trump and for Democrats’ response, saw the faceoff the same way.
  “With this president’s clear authoritarian bent, lack of respect for separation of powers and violations of the law, we’re in dangerous territory with still three-plus years to go. That’s what California has me concerned about,” Booker said.
  For months, Newsom angered many Democrats by inviting Trump-friendly figures onto his podcast or taking shots at his own party for going too far on the issue of transgender athletes playing in women’s sports.
  He tried to connect with Trump in an effort to get more federal money to rebuild after the devastation of the Los Angeles wildfires at the beginning of the year and suggested he’d work with Trump on tariffs aimed at bucking up the film industry that has been fleeing California, even as other leading Democrats called for more intense pushback, like when Illinois Gov. JB Pritzker said in a fiery speech in New Hampshire in April that, “never before in my life have I called for mass protests, for mobilization, for disruption. But I am now.”
  But the events of the last few days have rekindled the long-simmering rivalry between Trump and Newsom.
  Newsom dared the Trump administration in one television interview to arrest him rather than targeting immigrant children. Trump then suggested in response to a reporter’s question that Newsom should be arrested.
  The only rationale Trump has offered for making the threat of arresting a sitting governor is because “his primary crime is running for governor, because he’s done such a bad job.”
  “I like the fact that when one of Trump’s henchmen threatened Newsom with arrest, he said, ‘Well, come and get me, here I am.’ We’re not going to be afraid of Donald Trump because we have the rule of law on our side. We’re standing up for the Constitution. The states are not the pawns of the federal government. The states have an independent constitutional and political existence,” said Maryland Rep. Jamie Raskin, who taught constitutional law before being elected to the House. “Other governors should stand up for the rule of law and stand up for the rights of their people.”
  With some looters also taking to the streets while Trump and his deputy chief of staff refer to an “insurrection,” the situation hasn’t gone over well with every Democrat , including those who worry about playing into Trump’s hands on a signature issue.
  Pennsylvania Sen. John Fetterman is among members of the party who have called for a more forceful condemnation of violent protesters. Newsom himself has said that those engaged in violence or attacking police officers would be prosecuted and noted that law enforcement is already reviewing videos of the events to track down more perpetrators.
  Even before Trump already threatened “very heavy force” if any protesters disrupt the massive military parade he is hosting this Saturday in Washington on his 79 th birthday, leaders in other centers of immigrants were expressing concern about what happens if federal agents target their communities.
  “I would hope that New Yorkers will speak up and do whatever they believe is their constitutional right in a non-violent way, and if Trump tried to tamp it up, I think the people would see it for what it is,” said New York Rep. Greg Meeks. “I would say to New Yorkers and others, ‘We know what he’s trying to do.’”
  A few Republicans have joined Democrats in expressing concern, including swing district California GOP Rep. David Valadao, who tweeted Tuesday that he is “concerned about ongoing ICE operations through CA.” But for now, most Republican leaders have either been expressing support for Trump or staying quiet about the situation. House Speaker Mike Johnson said he couldn’t speak to the legal argument about arresting Newsom, but “he ought to be tarred and feathered.”
  While some Democratic strategists, including some who have kicked in with advice to Newsom in recent days, have urged a more defensive position that echoes Trump’s hardline approach to immigration so that they don’t give the president a fight he clearly wants, others are glad to see Newsom taking a more forceful lead on his own terms.
  “Democrats need to recognize that voters are appalled by Trump’s overreach on immigration – not just Democratic voters, but independent voters, libertarian leaning voters don’t believe in arresting random peaceful people and separating families,” said Texas Rep. Greg Casar, the chair of the Congressional Progressive Caucus. “So we shouldn’t be scared of going toe-to-toe with Trump on his overreach and abuse of people’s rights.”
```