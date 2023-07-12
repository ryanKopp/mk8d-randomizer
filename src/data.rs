pub const DRIVER_DATA: &str = "
name,speed,acceleration,weight,handling,traction
Baby Peach,2.5,4,2,5,4.25
Baby Daisy,2.5,4,2,5,4.25
Baby Rosalina,2.5,4.25,2,4.75,3.75
Lemmy,2.5,4.25,2,4.75,3.75
Baby Mario,2.75,4.25,2.25,4.5,4
Baby Luigi,2.75,4.25,2.25,4.5,4
Dry Bones,2.75,4.25,2.25,4.5,4
Koopa Troopa,3,4,2.5,4.5,4.25
Lakitu,3,4,2.5,4.5,4.25
Bowser Jr.,3,4,2.5,4.5,4.25
Toadette,3,4.25,2.5,4.25,3.5
Wendy,3,4.25,2.5,4.25,3.5
Isabelle,3,4.25,2.5,4.25,3.5
Toad,3.25,4,2.75,4.25,4
Shy Guy,3.25,4,2.75,4.25,4
Larry,3.25,4,2.75,4.25,4
Cat Peach,3.5,4,2.75,4,3.75
Inkling Girl,3.5,4,2.75,4,3.75
Villager Girl,3.5,4,2.75,4,3.75
Peach,3.75,3.75,3,3.75,3.75
Daisy,3.75,3.75,3,3.75,3.75
Yoshi,3.75,3.75,3,3.75,3.75
Birdo,3.75,3.75,3,3.75,3.75
Tanooki Mario,3.75,3.75,3.25,3.75,3.25
Inkling Boy,3.75,3.75,3.25,3.75,3.25
Villager Boy,3.75,3.75,3.25,3.75,3.25
Mario,4,3.5,3.5,3.5,3.5
Ludwig,4,3.5,3.5,3.5,3.5
Luigi,4,3.5,3.5,3.75,3.25
Iggy,4,3.5,3.5,3.75,3.25
Kamek,4,3.5,3.5,3.75,3.25
Rosalina,4.25,3.25,3.75,3.25,3.75
King Boo,4.25,3.25,3.75,3.25,3.75
Link,4.25,3.25,3.75,3.25,3.75
Metal Mario,4.25,3.25,4.5,3.25,3.25
Pink Gold Peach,4.25,3.25,4.5,3.25,3.25
Petey Pirahna,4.25,3.25,4.5,3.25,3.25
Donkey Kong,4.5,3.25,4,3,3
Waluigi,4.5,3.25,4,3,3
Roy,4.5,3.25,4,3,3
Wiggler,4.5,3.25,4,3,3
Wario,4.75,3,4.25,2.75,3.25
Dry Bowser,4.75,3,4.25,2.75,3.25
Bowser,4.75,3,4.5,2.5,3
Morton,4.75,3,4.5,2.5,3";

pub const VEHICLE_DATA: &str = "
name,speed,acceleration,weight,handling,traction
Standard Kart,0,0,0,0,0
Pipe Frame,-0.25,0.5,-0.25,0.5,0.25
Mach 8,0,-0.25,0.25,-0.25,0.25
Steel Driver,0.25,-0.75,0.5,-0.5,0
Cat Cruiser,-0.25,0.25,0,0.25,0
Circuit Special,0.5,-0.75,0.25,-0.5,-0.5
Tri-Speeder,0.25,-0.75,0.5,-0.5,0
Badwagon,0.5,-1,0.5,-0.75,0.5
Prancer,0.25,-0.5,-0.25,0,-0.25
Biddybuggy,-0.75,0.75,-0.5,0.5,0.25
Landship,-0.25,0.5,-0.5,0.25,0.75
Sneeker,0.25,-0.5,0,0,-0.75
Sports Coupé,0,-0.25,0.25,-0.25,0.25
Gold Standard,0.25,-0.5,0,0,-0.75
GLA,0.5,-1,0.5,-0.75,0.5
W 25 Silver Arrow,-0.25,0.25,-0.25,0.25,0.5
300 SL Roadster,0,0,0,0,0
Blue Falcon,0.25,-0.25,-0.5,-0.25,0
Tanooki Kart,0,-0.5,0.25,0.25,1
B Dasher,0.5,-0.75,0.25,-0.5,-0.5
Streetle,-0.25,0.5,-0.5,0.25,0.75
P-Wing,0.5,-0.75,0.25,-0.5,-0.5
Koopa Clown,0,-0.5,0.25,0.25,1
Standard Bike,-0.25,0.25,-0.25,0.25,0.5
Comet+,-0.25,0.25,0,0.25,0
Sport Bike+,0.25,-0.5,-0.25,0,-0.25
The Duke,0,0,0,0,0
Flame Rider,-0.25,0.25,-0.25,0.25,0.5
Varmint,-0.25,0.5,-0.25,0.5,0.25
Mr. Scooty,-0.75,0.75,-0.5,0.5,0.25
Jet Bike+,0.25,-0.5,-0.25,0,-0.25
Yoshi Bike+,-0.25,0.25,0,0.25,0
Master Cycle+,0.25,-0.5,0,0,-0.75
City Tripper,-0.25,0.5,-0.25,0.5,0.25
Standard ATV,0.5,-1,0.5,-0.75,0.5
Wild Wiggler,-0.25,0.25,-0.25,0.25,0.5
Teddy Buggy,-0.25,0.25,0,0.25,0
Bone Rattler,0.25,-0.75,0.5,-0.5,0
Splat Buggy,0.25,-0.25,-0.5,-0.25,0
Inkstriker,0,-0.25,0.25,-0.25,0.25
Master Cycle Zero,0,-0.5,0.25,0.25,1";

pub const TIRE_DATA: &str = "
name,speed,acceleration,weight,handling,traction
Standard,0,0,0,0,0
Monster,0.25,-0.5,0.5,-0.75,0.5
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
Hot Monster,0.25,-0.5,0.5,-0.75,0.5
Azure Roller,-0.5,0.5,-0.5,0.25,-0.25
Crimson Slim,0.25,-0.5,0,0.25,-1
Cyber Slick,0.5,-0.75,0.25,-0.25,-1.25
Retro Off-Road,0.25,-0.25,0.25,-0.5,0.25
Gold Tires,0.5,-1,0.5,-0.25,-0.75
GLA Tires,0,0,0,0,0
Triforce Tires,0.25,-0.25,0.25,-0.5,0.25
Leaf Tires,-0.25,0.25,-0.5,0,-0.5
Ancient Tires,0.25,-0.5,0.5,-0.75,0.5";

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
Mario Circuit
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
Rainbow Road
Yoshi Circuit (GCN)
Excitebike Arena
Dragon Driftway
Mute City
Baby Park (GCN)
Cheese Land (GBA)
Wild Woods
Animal Crossing
Moo Moo Meadows (Wii)
Mario Circuit (GBA)
Cheep Cheep Beach (DS)
Toad's Turnpike (N64)
Dry Dry Desert (GCN)
Donut Plains 3 (SNES)
Royal Raceway (N64)
DK Jungle (3DS)
Wario Stadium (DS)
Sherbet Land (GCN)
Music Park (3DS)
Yoshi Valley (N64)
Tick-Tock Clock (DS)
Piranha Plant Slide (3DS)
Grumble Volcano (Wii)
Rainbow Road (N64)
Wario's Gold Mine (Wii)
Rainbow Road (SNES)
Ice Ice Outpost
Hyrule Circuit
Neo Bowser City (3DS)
Ribbon Road (GBA)
Super Bell Subway
Big Blue
Paris Promenade (Tour)
Toad Circuit (3DS)
Choco Mountain (N64)
Coconut Mall (Wii)
Tokyo Blur (Tour)
Shroom Ridge (DS)
Sky Garden (GBA)
Ninja Hideaway
New York Minute (Tour)
Mario Circuit 3 (SNES)
Kalimari Desert (N64)
Waluigi Pinball (DS)
Sydney Sprint (Tour)
Snow Land (GBA)
Mushroom Gorge (Wii)
Sky-High Sundae
London Loop (Tour)
Boo Lake (GBA)
Rock Rock Mountain (3DS)
Maple Treeway (Wii)
Berlin Byways (Tour)
Peach Gardens (DS)
Merry Mountain
Rainbow Road (3DS)
Amsterdam Drift (Tour)
Riverside Park (GBA)
DK Summit (Wii)
Yoshi's Island
Bangkok Rush (Tour)
Mario Circuit (DS)
Waluigi Stadium (GCN)
Singapore Speedway (Tour)
Athens Dash (Tour)
Daisy Cruiser (GCN)
Moonview Highway (Wii)
Squeaky Clean Sprint
Los Angeles Laps (Tour)
Sunset Wilds (GBA)
Koopa Cape (Wii)
Vancouver Velocity (Tour)";
