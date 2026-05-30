cd 
rm -rf animals 
mkdir -p animals/{mammals,reptiles,birds} 
touch animals/mammals/{lion,tiger,elephant}.txt 
touch animals/reptiles/{snake,crocodile,turtle}.txt 
touch animals/birds/{eagle,parrot,penguin}.txt

cat > animals/mammals/lion.txt <<EOF
Name: Lion
Category: Mammal
Habitat: Savanna
Diet: Carnivore
Lifespan: 10-14 years
Speed: Up to 80 km/h
Fact: Lions live in groups called prides.
EOF

cat > animals/mammals/tiger.txt <<EOF
Name: Tiger
Category: Mammal
Habitat: Forest
Diet: Carnivore
Lifespan: 10-15 years
Speed: Up to 65 km/h
Fact: Tigers are excellent swimmers.
EOF

cat > animals/mammals/elephant.txt <<EOF
Name: Elephant
Category: Mammal
Habitat: Grassland
Diet: Herbivore
Lifespan: 60-70 years
Weight: Up to 6000 kg
Fact: Elephants have strong memories.
EOF

cat > animals/reptiles/snake.txt <<EOF
Name: Snake
Category: Reptile
Habitat: Various
Diet: Carnivore
Lifespan: 9-20 years
Movement: Slithering
Fact: Some snakes can detect heat.
EOF

cat > animals/reptiles/crocodile.txt <<EOF
Name: Crocodile
Category: Reptile
Habitat: Rivers and wetlands
Diet: Carnivore
Lifespan: 70-100 years
Length: Up to 7 meters
Fact: Crocodiles are ancient predators.
EOF

cat > animals/reptiles/turtle.txt <<EOF
Name: Turtle
Category: Reptile
Habitat: Oceans and land
Diet: Omnivore
Lifespan: 50-100 years
Protection: Hard shell
Fact: Some turtles migrate thousands of kilometers.
EOF

cat > animals/birds/eagle.txt <<EOF
Name: Eagle
Category: Bird
Habitat: Mountains and forests
Diet: Carnivore
Lifespan: 20-30 years
Vision: Exceptional eyesight
Fact: Eagles can spot prey from great distances.
EOF

cat > animals/birds/parrot.txt <<EOF
Name: Parrot
Category: Bird
Habitat: Tropical regions
Diet: Omnivore
Lifespan: 20-80 years
Ability: Mimics sounds
Fact: Some parrots learn hundreds of words.
EOF

cat > animals/birds/penguin.txt <<EOF
Name: Penguin
Category: Bird
Habitat: Polar regions
Diet: Carnivore
Lifespan: 15-20 years
Movement: Swimming
Fact: Penguins cannot fly but are excellent swimmers.
EOF
