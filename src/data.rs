pub const DRIVER_DATA: &str = "
name,speed,acceleration,weight,handling,traction
Baby Peach,2.25,4,2,5,4.25
Baby Daisy,2.25,4,2,5,4.25
Baby Rosalina,2.25,4.25,2,4.75,3.75
Lemmy,2.25,4.25,2,4.75,3.75
Baby Mario,2.5,4.25,2.25,4.5,4
Baby Luigi,2.5,4.25,2.25,4.5,4
Dry Bones,2.5,4.25,2.25,4.5,4
Koopa Troopa,2.75,4,2.5,4.5,4.25
Lakitu,2.75,4,2.5,4.5,4.25
Bowser Jr.,2.75,4,2.5,4.5,4.25
Toadette,2.75,4.25,2.5,4.25,3.5
Wendy,2.75,4.25,2.5,4.25,3.5
Isabelle,2.75,4.25,2.5,4.25,3.5
Toad,3,4,2.75,4.25,4
Shy Guy,3,4,2.75,4.25,4
Larry,3,4,2.75,4.25,4
Cat Peach,3.25,4,2.75,4,3.75
Inkling (F),3.25,4,2.75,4,3.75
Villager (F),3.25,4,2.75,4,3.75
Peach,3.5,3.75,3,3.75,3.75
Daisy,3.5,3.75,3,3.75,3.75
Yoshi,3.5,3.75,3,3.75,3.75
Tanooki Mario,3.5,3.75,3.25,3.75,3.25
Inkling (M),3.5,3.75,3.25,3.75,3.25
Villager (M),3.5,3.75,3.25,3.75,3.25
Luigi,3.75,3.5,3.5,3.75,3.25
Iggy,3.75,3.5,3.5,3.75,3.25
Mario,3.75,3.5,3.5,3.5,3.5
Ludwig,3.75,3.5,3.5,3.5,3.5
Metal Mario,4.25,3.25,4.5,3.25,3.25
Pink Gold Peach,4.25,3.25,4.5,3.25,3.25
Rosalina,4,3.25,3.75,3.25,3.75
King Boo,4,3.25,3.75,3.25,3.75
Link,4,3.25,3.75,3.25,3.75
Donkey Kong,4.5,3.25,4,3,3
Waluigi,4.5,3.25,4,3,3
Roy,4.5,3.25,4,3,3
Wario,4.75,3,4.25,2.75,3.25
Dry Bowser,4.75,3,4.25,2.75,3.25
Bowser,4.75,3,4.5,2.5,3
Morton,4.75,3,4.5,2.5,3";

pub const VEHICLE_DATA: &str = "
name,speed,acceleration,weight,handling,traction
Standard Kart,0,0,0,0,0
Pipe Frame,-0.5,0.5,-0.25,0.5,0.25
Mach 8,0,-0.25,0.25,-0.25,0.25
Steel Driver,0.25,-0.75,0.5,-0.5,0
Cat Cruiser,-0.25,0.25,0,0.25,0
Circuit Special,0.5,-0.75,0.25,-0.5,-0.5
Tri-Speeder,0.25,-0.75,0.5,-0.5,0
Badwagon,0.5,-1,0.5,-0.75,0.5
Prancer,0.25,-0.5,-0.25,0,-0.25
Biddybuggy,-0.75,0.75,-0.5,0.5,0.25
Landship,-0.5,0.5,-0.5,0.25,0.75
Sneeker,0.25,-0.5,0,0,-0.75
Sports Coupé,0,-0.25,0.25,-0.25,0.25
Gold Standard,0.25,-0.5,0,0,-0.75
GLA,0.5,-1,0.5,-0.75,0.5
W 25 Silver Arrow,-0.25,0.25,-0.25,0.25,0.5
300 SL Roadster,0,0,0,0,0
Blue Falcon,0.25,-0.25,-0.5,-0.25,0
Tanooki Kart,-0.25,-0.5,0.25,0.25,1
B Dasher,0.5,-0.75,0.25,-0.5,-0.5
Streetle,-0.5,0.5,-0.5,0.25,0.75
P-Wing,0.5,-0.75,0.25,-0.5,-0.5
Koopa Clown,-0.25,-0.5,0.25,0.25,1
Standard Bike,-0.25,0.25,-0.25,0.25,0.5
Comet+,-0.25,0.25,0,0.25,0
Sport Bike+,0.25,-0.5,-0.25,0,-0.25
The Duke,0,0,0,0,0
Flame Rider,-0.25,0.25,-0.25,0.25,0.5
Varmint,-0.5,0.5,-0.25,0.5,0.25
Mr. Scooty,-0.75,0.75,-0.5,0.5,0.25
Jet Bike+,0.25,-0.5,-0.25,0,-0.25
Yoshi Bike+,-0.25,0.25,0,0.25,0
Master Cycle+,0.25,-0.5,0,0,-0.75
City Tripper,-0.5,0.5,-0.25,0.5,0.25
Standard Quad,0.5,-1,0.5,-0.75,0.5
Wild Wiggler,-0.25,0.25,-0.25,0.25,0.5
Teddy Buggy,-0.25,0.25,0,0.25,0
Bone Rattler,0.25,-0.75,0.5,-0.5,0
Splat Buggy,0.25,-0.25,-0.5,-0.25,0
Inkstriker,0,-0.25,0.25,-0.25,0.25
Master Cycle Zero,-0.25,-0.5,0.25,0.25,1";

pub const TIRE_DATA: &str = "
name,speed,acceleration,weight,handling,traction
Standard,0,0,0,0,0
Monster,0,-0.5,0.5,-0.75,0.5
Roller,-0.5,0.5,-0.5,0.25,-0.25
Slim,0.25,-0.5,0,0.25,-1
Slick,0.5,-0.75,0.25,-0.25,-1.25
Metal,0.5,-1,0.5,-0.25,-0.75
Button,-0.25,0.25,-0.5,0,-0.5
Off-Road,0.25,-0.25,0.25,-0.5,0.25
Sponge,-0.25,0,-0.25,-0.25,0.25
Wood,0.25,-0.5,0,0.25,-1
Cushion,-0.25,0,-0.25,-0.25,0.25
Blue Standard,0,0,0,0,0
Hot Monster,0,-0.5,0.5,-0.75,0.5
Azure Roller,-0.5,0.5,-0.5,0.25,-0.25
Crimson Slim,0.25,-0.5,0,0.25,-1
Cyber Slick,0.5,-0.75,0.25,-0.25,-1.25
Retro Off-Road,0.25,-0.25,0.25,-0.5,0.25
Gold Tires,0.5,-1,0.5,-0.25,-0.75
GLA Tires,0,0,0,0,0
Triforce Tires,0.25,-0.25,0.25,-0.5,0.25
Leaf Tires,-0.25,0.25,-0.5,0,-0.5
Ancient Tires,0,-0.5,0.5,-0.75,0.5";

pub const GLIDER_DATA: &str = "
name,speed,acceleration,weight,handling,traction
Super Glider,0,0,0,0,0
Cloud Glider,-0.25,0.25,-0.25,0,0
Wario Wing,0,0,0.25,0,-0.25
Waddle Wing,0,0,0,0,0
Peach Parasol,-0.25,0.25,0,0,-0.25
Parachute,-0.25,0.25,-0.25,0,0
Parafoil,-0.25,0.25,0,0,-0.25
Flower Glider,-0.25,0.25,-0.25,0,0
Bowser Kite,-0.25,0.25,0,0,-0.25
Plane Glider,0,0,0.25,0,-0.25
MKTV Parafoil,-0.25,0.25,0,0,-0.25
Gold Glider,0,0,0.25,0,-0.25
Hylian Kite,0,0,0,0,0
Paper Glider,-0.25,0.25,-0.25,0,0
Paraglider,0,0,0.25,0,-0.25";

pub const MAPS: &str = "
name
Mario Kart Stadium
Water Park
Sweet Sweet Canyon
Thwomp Ruins
Mario Circuit (MK8)
Toad Harbor
Twisted Mansion
Shy Guy Falls
Sunshine Airport
Dolphin Shoals
Electrodome
Mount Wario
Cloudtop Cruise
Bone-Dry Dunes
Bowser's Castle
Rainbow Road (MK8)
Yoshi Circuit
Excitebike Arena
Dragon Driftway
Mute City
Baby Park
Cheese Land
Wild Woods
Animal Crossing
Moo Moo Meadows
Mario Circuit (GBA)
Cheep Cheep Beach
Toad's Turnpike
Dry Dry Desert
Donut Plains 3
Royal Raceway
DK Jungle
Wario Stadium
Sherbet Land
Music Park
Yoshi Valley
Tick-Tock Clock
Piranha Plant Slide
Grumble Volcano
Rainbow Road (N64)
Wario's Gold Mine
Rainbow Road (SNES)
Ice Ice Outpost
Hyrule Circuit
Neo Bowser City
Ribbon Road
Super Bell Subway
Big Blue
Paris Promenade
Toad Circuit
Choco Mountain
Coconut Mall
Tokyo Blur
Shroom Ridge
Sky Garden
Ninja Hideaway
New York Minute
Mario Circuit 3 (SNES)
Kalimari Desert
Waluigi Pinball
Sydney Sprint
Snow Land
Mushroom Gorge
Sky-High Sundae
London Loop
Boo Lake
Rock Rock Mountain
Maple Treeway
Berlin Byways
Peach Gardens
Merry Mountain
Rainbow Road (3DS)";
