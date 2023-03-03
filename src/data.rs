pub const DRIVER_DATA: &str = "
name,speed,acceleration,weight,handling,traction
Character: Baby Peach,2.25,4,2,5,4.25
Character: Baby Daisy,2.25,4,2,5,4.25
Character: Baby Rosalina,2.25,4.25,2,4.75,3.75
Character: Lemmy,2.25,4.25,2,4.75,3.75
Character: Baby Mario,2.5,4.25,2.25,4.5,4
Character: Baby Luigi,2.5,4.25,2.25,4.5,4
Character: Dry Bones,2.5,4.25,2.25,4.5,4
Character: Koopa Troopa,2.75,4,2.5,4.5,4.25
Character: Lakitu,2.75,4,2.5,4.5,4.25
Character: Bowser Jr.,2.75,4,2.5,4.5,4.25
Character: Toadette,2.75,4.25,2.5,4.25,3.5
Character: Wendy,2.75,4.25,2.5,4.25,3.5
Character: Isabelle,2.75,4.25,2.5,4.25,3.5
Character: Toad,3,4,2.75,4.25,4
Character: Shy Guy,3,4,2.75,4.25,4
Character: Larry,3,4,2.75,4.25,4
Character: Cat Peach,3.25,4,2.75,4,3.75
Character: Inkling (F),3.25,4,2.75,4,3.75
Character: Villager (F),3.25,4,2.75,4,3.75
Character: Peach,3.5,3.75,3,3.75,3.75
Character: Daisy,3.5,3.75,3,3.75,3.75
Character: Yoshi,3.5,3.75,3,3.75,3.75
Character: Tanooki Mario,3.5,3.75,3.25,3.75,3.25
Character: Inkling (M),3.5,3.75,3.25,3.75,3.25
Character: Villager (M),3.5,3.75,3.25,3.75,3.25
Character: Luigi,3.75,3.5,3.5,3.75,3.25
Character: Iggy,3.75,3.5,3.5,3.75,3.25
Character: Mario,3.75,3.5,3.5,3.5,3.5
Character: Ludwig,3.75,3.5,3.5,3.5,3.5
Character: Metal Mario,4.25,3.25,4.5,3.25,3.25
Character: Pink Gold Peach,4.25,3.25,4.5,3.25,3.25
Character: Rosalina,4,3.25,3.75,3.25,3.75
Character: King Boo,4,3.25,3.75,3.25,3.75
Character: Link,4,3.25,3.75,3.25,3.75
Character: Donkey Kong,4.5,3.25,4,3,3
Character: Waluigi,4.5,3.25,4,3,3
Character: Roy,4.5,3.25,4,3,3
Character: Wario,4.75,3,4.25,2.75,3.25
Character: Dry Bowser,4.75,3,4.25,2.75,3.25
Character: Bowser,4.75,3,4.5,2.5,3
Character: Morton,4.75,3,4.5,2.5,3";

pub const VEHICLE_DATA: &str = "
name,speed,acceleration,weight,handling,traction
Kart:      Standard Kart,0,0,0,0,0
Kart:      Pipe Frame,-0.5,0.5,-0.25,0.5,0.25
Kart:      Mach 8,0,-0.25,0.25,-0.25,0.25
Kart:      Steel Driver,0.25,-0.75,0.5,-0.5,0
Kart:      Cat Cruiser,-0.25,0.25,0,0.25,0
Kart:      Circuit Special,0.5,-0.75,0.25,-0.5,-0.5
Kart:      Tri-Speeder,0.25,-0.75,0.5,-0.5,0
Kart:      Badwagon,0.5,-1,0.5,-0.75,0.5
Kart:      Prancer,0.25,-0.5,-0.25,0,-0.25
Kart:      Biddybuggy,-0.75,0.75,-0.5,0.5,0.25
Kart:      Landship,-0.5,0.5,-0.5,0.25,0.75
Kart:      Sneeker,0.25,-0.5,0,0,-0.75
Kart:      Sports Coupé,0,-0.25,0.25,-0.25,0.25
Kart:      Gold Standard,0.25,-0.5,0,0,-0.75
Kart:      GLA,0.5,-1,0.5,-0.75,0.5
Kart:      W 25 Silver Arrow,-0.25,0.25,-0.25,0.25,0.5
Kart:      300 SL Roadster,0,0,0,0,0
Kart:      Blue Falcon,0.25,-0.25,-0.5,-0.25,0
Kart:      Tanooki Kart,-0.25,-0.5,0.25,0.25,1
Kart:      B Dasher,0.5,-0.75,0.25,-0.5,-0.5
Kart:      Streetle,-0.5,0.5,-0.5,0.25,0.75
Kart:      P-Wing,0.5,-0.75,0.25,-0.5,-0.5
Kart:      Koopa Clown,-0.25,-0.5,0.25,0.25,1
Kart:      Standard Bike,-0.25,0.25,-0.25,0.25,0.5
Kart:      Comet+,-0.25,0.25,0,0.25,0
Kart:      Sport Bike+,0.25,-0.5,-0.25,0,-0.25
Kart:      The Duke,0,0,0,0,0
Kart:      Flame Rider,-0.25,0.25,-0.25,0.25,0.5
Kart:      Varmint,-0.5,0.5,-0.25,0.5,0.25
Kart:      Mr. Scooty,-0.75,0.75,-0.5,0.5,0.25
Kart:      Jet Bike+,0.25,-0.5,-0.25,0,-0.25
Kart:      Yoshi Bike+,-0.25,0.25,0,0.25,0
Kart:      Master Cycle+,0.25,-0.5,0,0,-0.75
Kart:      City Tripper,-0.5,0.5,-0.25,0.5,0.25
Kart:      Standard Quad,0.5,-1,0.5,-0.75,0.5
Kart:      Wild Wiggler,-0.25,0.25,-0.25,0.25,0.5
Kart:      Teddy Buggy,-0.25,0.25,0,0.25,0
Kart:      Bone Rattler,0.25,-0.75,0.5,-0.5,0
Kart:      Splat Buggy,0.25,-0.25,-0.5,-0.25,0
Kart:      Inkstriker,0,-0.25,0.25,-0.25,0.25
Kart:      Master Cycle Zero,-0.25,-0.5,0.25,0.25,1";

pub const TIRE_DATA: &str = "
name,speed,acceleration,weight,handling,traction
Tires:     Standard,0,0,0,0,0
Tires:     Monster,0,-0.5,0.5,-0.75,0.5
Tires:     Roller,-0.5,0.5,-0.5,0.25,-0.25
Tires:     Slim,0.25,-0.5,0,0.25,-1
Tires:     Slick,0.5,-0.75,0.25,-0.25,-1.25
Tires:     Metal,0.5,-1,0.5,-0.25,-0.75
Tires:     Button,-0.25,0.25,-0.5,0,-0.5
Tires:     Off-Road,0.25,-0.25,0.25,-0.5,0.25
Tires:     Sponge,-0.25,0,-0.25,-0.25,0.25
Tires:     Wood,0.25,-0.5,0,0.25,-1
Tires:     Cushion,-0.25,0,-0.25,-0.25,0.25
Tires:     Blue Standard,0,0,0,0,0
Tires:     Hot Monster,0,-0.5,0.5,-0.75,0.5
Tires:     Azure Roller,-0.5,0.5,-0.5,0.25,-0.25
Tires:     Crimson Slim,0.25,-0.5,0,0.25,-1
Tires:     Cyber Slick,0.5,-0.75,0.25,-0.25,-1.25
Tires:     Retro Off-Road,0.25,-0.25,0.25,-0.5,0.25
Tires:     Gold Tires,0.5,-1,0.5,-0.25,-0.75
Tires:     GLA Tires,0,0,0,0,0
Tires:     Triforce Tires,0.25,-0.25,0.25,-0.5,0.25
Tires:     Leaf Tires,-0.25,0.25,-0.5,0,-0.5
Tires:     Ancient Tires,0,-0.5,0.5,-0.75,0.5";

pub const GLIDER_DATA: &str = "
name,speed,acceleration,weight,handling,traction
Glider:    Super Glider,0,0,0,0,0
Glider:    Cloud Glider,-0.25,0.25,-0.25,0,0
Glider:    Wario Wing,0,0,0.25,0,-0.25
Glider:    Waddle Wing,0,0,0,0,0
Glider:    Peach Parasol,-0.25,0.25,0,0,-0.25
Glider:    Parachute,-0.25,0.25,-0.25,0,0
Glider:    Parafoil,-0.25,0.25,0,0,-0.25
Glider:    Flower Glider,-0.25,0.25,-0.25,0,0
Glider:    Bowser Kite,-0.25,0.25,0,0,-0.25
Glider:    Plane Glider,0,0,0.25,0,-0.25
Glider:    MKTV Parafoil,-0.25,0.25,0,0,-0.25
Glider:    Gold Glider,0,0,0.25,0,-0.25
Glider:    Hylian Kite,0,0,0,0,0
Glider:    Paper Glider,-0.25,0.25,-0.25,0,0
Glider:    Paraglider,0,0,0.25,0,-0.25";

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
