#!/bin/sh
echo "This extractor is made for version 022920"
set -x
set -e
mkdir -p images
# Extract image
#pdfimages -all Iron\ Helm\ PNP_*.pdf images/iron_helm
# Remove smasks
#rm images/iron_helm-[0-9][0-9][13579].jpg
#pdfimages -list Iron\ Helm\ PNP_022920.pdf | grep smask | column -t | awk '{print $2}' | xargs -I '{n}' printf '%03d\n' {n} | xargs -I '{n}' rm images/iron_helm-{n}.jpg

ls images/*.png | sed s/.png// | xargs -I "{f}" convert {f}.png {f}.jpg

pushd images

### CARDBACKS
mkdir -p cardbacks
pushd cardbacks
cp ../iron_helm-000.jpg characters.jpg
cp ../iron_helm-002.jpg dungeon.jpg
cp ../iron_helm-004.jpg bosses.jpg
cp ../iron_helm-066.jpg enemies.jpg
cp ../iron_helm-160.jpg loot.jpg
cp ../iron_helm-224.jpg potions.jpg
cp ../iron_helm-256.jpg trappings.jpg
cp ../iron_helm-320.jpg plots.jpg
cp ../iron_helm-352.jpg skills.jpg
popd

### MISC
mkdir -p misc
pushd misc
cp ../iron_helm-290.jpg morality_tracker.jpg
cp ../iron_helm-294.jpg map_square.jpg
cp ../iron_helm-308.jpg map_cavy.jpg
popd

### TOKENS
mkdir -p tokens
pushd tokens
cp ../iron_helm-404.jpg ration.jpg
cp ../iron_helm-424.jpg energy.jpg
cp ../iron_helm-452.jpg poison.jpg
cp ../iron_helm-472.jpg heart.jpg
cp ../iron_helm-604.jpg coin.jpg
# Now the blessing tokens
# some of them are just bigger versions of the normal tokens
# I ignored those.
cp ../iron_helm-646.jpg double_heart.jpg
cp ../iron_helm-684.jpg blessing.jpg
popd

### BOSSES
mkdir -p bosses
pushd bosses
cp ../iron_helm-016.jpg lich.jpg
cp ../iron_helm-020.jpg lurker.jpg
cp ../iron_helm-024.jpg naga.jpg
popd

### ENEMIES
mkdir -p enemies
pushd enemies
cp ../iron_helm-094.jpg consuming_mass.jpg
cp ../iron_helm-112.jpg fishman.jpg
cp ../iron_helm-114.jpg skinkling.jpg # 2 times
cp ../iron_helm-116.jpg flying_snake.jpg # 2 times
cp ../iron_helm-118.jpg cave_troll.jpg
cp ../iron_helm-120.jpg goblin.jpg
cp ../iron_helm-122.jpg undead_archer.jpg # 2 times
cp ../iron_helm-124.jpg orc_warrior.jpg
cp ../iron_helm-126.jpg undead_warrior.jpg
cp ../iron_helm-144.jpg wolf.jpg # 2 times
cp ../iron_helm-148.jpg wraith.jpg
cp ../iron_helm-152.jpg zombie.jpg # 2 times
cp ../iron_helm-156.jpg goblin.jpg
popd

### CHARACTERS
mkdir -p characters
pushd characters
cp ../iron_helm-018.jpg moliclan.jpg
cp ../iron_helm-022.jpg sortab.jpg # my fav
cp ../iron_helm-026.jpg zolla.jpg
cp ../iron_helm-028.jpg fayon.jpg
popd

### DUNGEON CARDS
mkdir -p dungeon
pushd dungeon
cp ../iron_helm-030.jpg altar.jpg
cp ../iron_helm-048.jpg ambush.jpg # 3 times
cp ../iron_helm-050.jpg labyrinth.jpg
cp ../iron_helm-052.jpg arrow_trap.jpg
cp ../iron_helm-054.jpg merchant.jpg
cp ../iron_helm-056.jpg campsite.jpg
cp ../iron_helm-058.jpg mushroom_grove.jpg
cp ../iron_helm-060.jpg clearing.jpg
cp ../iron_helm-062.jpg skirmish.jpg # 4 times
cp ../iron_helm-080.jpg treasure.jpg
popd

### LOOT
mkdir -p loot
pushd loot
cp ../iron_helm-178.jpg elven_bow.jpg
cp ../iron_helm-180.jpg cinderblade.jpg
cp ../iron_helm-182.jpg explorers_map.jpg
cp ../iron_helm-184.jpg cookbook.jpg
cp ../iron_helm-186.jpg field_guide.jpg
cp ../iron_helm-188.jpg dagger.jpg
cp ../iron_helm-190.jpg health_potion.jpg # potion with loot cardback
cp ../iron_helm-208.jpg iceback.jpg
cp ../iron_helm-210.jpg magic_ring.jpg
cp ../iron_helm-212.jpg ../iron_helm.jpg
cp ../iron_helm-214.jpg mimic.jpg # enemy with loot cardback
cp ../iron_helm-216.jpg lamp.jpg
cp ../iron_helm-218.jpg ration.jpg # 2 times
cp ../iron_helm-220.jpg long_sword.jpg
cp ../iron_helm-222.jpg scale_armor.jpg
cp ../iron_helm-240.jpg shield.jpg
cp ../iron_helm-244.jpg warhammer.jpg
popd

### POTIONS
mkdir -p potions
pushd potions
# All these potions exist exactly twice!
cp ../iron_helm-242.jpg holy_water.jpg
cp ../iron_helm-242.jpg ice_shard.jpg
cp ../iron_helm-250.jpg spark_bomb.jpg
cp ../iron_helm-252.jpg health.jpg
cp ../iron_helm-254.jpg antidote.jpg
popd

### TRAPPINGS
mkdir -p trappings
pushd trappings
cp ../iron_helm-272.jpg door_wedge.jpg
cp ../iron_helm-274.jpg ../iron_helm.jpg
cp ../iron_helm-276.jpg leather_armor.jpg
cp ../iron_helm-278.jpg dagger.jpg
cp ../iron_helm-280.jpg mace.jpg
cp ../iron_helm-282.jpg axe.jpg
cp ../iron_helm-284.jpg buckler.jpg
cp ../iron_helm-286.jpg circlet.jpg
cp ../iron_helm-304.jpg torch.jpg
cp ../iron_helm-306.jpg wooden_staff.jpg
cp ../iron_helm-310.jpg short_bow.jpg
cp ../iron_helm-314.jpg rations.jpg # plurality is not a mistake
cp ../iron_helm-316.jpg robes.jpg
cp ../iron_helm-318.jpg shield.jpg
popd

### PLOTS
mkdir -p plots
pushd plots
cp ../iron_helm-336.jpg the_pigman.jpg
cp ../iron_helm-338.jpg the_prisoner.jpg
cp ../iron_helm-340.jpg the_shrine.jpg
cp ../iron_helm-342.jpg the_passageway.jpg
cp ../iron_helm-344.jpg the_traveler.jpg
cp ../iron_helm-346.jpg hidd_inn.jpg
cp ../iron_helm-348.jpg the_corpse.jpg
cp ../iron_helm-350.jpg the_old_woman.jpg
cp ../iron_helm-378.jpg the_wolf.jpg
popd

### SKILLS
mkdir -p skills
pushd plots
cp ../iron_helm-368.jpg berserk.jpg
cp ../iron_helm-370.jpg conceal.jpg
cp ../iron_helm-372.jpg divinity.jpg
cp ../iron_helm-374.jpg archery.jpg
cp ../iron_helm-376.jpg dodge.jpg
cp ../iron_helm-380.jpg alchemy.jpg
cp ../iron_helm-382.jpg appraise.jpg
cp ../iron_helm-394.jpg shield_block.jpg
cp ../iron_helm-396.jpg shadow.jpg
cp ../iron_helm-398.jpg dual_wield.jpg
cp ../iron_helm-400.jpg herbalism.jpg
cp ../iron_helm-402.jpg parry.jpg
popd

rm *.jpg
rm *.png

# Ignoring the ugly layout cards you're supposed to place the decks on

popd
