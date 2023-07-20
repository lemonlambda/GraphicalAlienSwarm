nix build
rm -rf alien_swarm
mkdir alien_swarm
cp result/bin/* alien_swarm/
cp -r assets alien_swarm
zip -r ./alien_swarm.zip ./alien_swarm
