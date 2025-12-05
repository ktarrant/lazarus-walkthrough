# University Student Rewards

Complete each local Pokédex to claim the reward from the nearby student researcher.

- <label><input type="checkbox" class="quest-check" data-quest="uni-acrisia-city"> Acrisia City: 15 Nest Balls</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-bronze-fields"> Bronze Fields: Bright Powder</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-jusmail-town"> Jusmail Town: Casual Coral Outfit</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-acrisia-mountains"> Acrisia Mountains: 15 Moon Balls</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-riverwalk-trail"> Riverwalk Trail: Eviolite</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-kalami-city"> Kalami City: TM Whirlpool & East Blue Ilios Outfit (if not already unlocked)</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-wanderers-woods"> Wanderer’s Woods: TM Deepwater Curse</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-pythios-town"> Pythios Town: Traditional Outfit</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-erinys-path"> Erinys Path: 2 Focus Sash</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-asfal-hills"> Asfal Hills: 15 Quick Balls</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-sofos-city"> Sofos City: Amulet Coin & Old Sofos Toga Outfit (if not already unlocked)</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-kipos-town"> Kipos Town: Pokémon Box Link (Portable PC)</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-sea-of-asteri"> Sea of Asteri: 2 Life Orbs</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-marmaro-island"> Marmaro Island: 20 Dive Balls</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-myrrini-island"> Myrrini Island: TM Poison Jab</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-pentepetal-city"> Péntepetal City: Big Lapras Doll (Cabin decoration)</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-sea-of-vulcai"> Sea of Vulcai: Assault Vest</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-fresco-isles"> The Fresco Isles: Safety Goggles</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-palati-city"> Palati City: TM Roost</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-kaptara-island"> Kaptara Island: TM Earthquake</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-lastlight-road"> Lastlight Road: 20 Timer Balls</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-tower-of-dioxippus"> Tower of Dioxippus: Gyarados Doll</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-corrin-crossing"> Corrin Crossing: TM Dazzling Gleam</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-port-pello"> Port Pello: Ancient War Armor Outfit</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-pollen-road"> Pollen Road: 3 EXP Candy XL</label>
- <label><input type="checkbox" class="quest-check" data-quest="uni-davosi-strait"> Davosi Strait: Sacred Ash</label>

<script>
(function() {
  if (window.__lazarusQuestInit) return; window.__lazarusQuestInit = true;
  const KEY = 'lazarusQuests';
  function load() { try { return JSON.parse(localStorage.getItem(KEY) || '{}'); } catch (_) { return {}; } }
  function save(state) { try { localStorage.setItem(KEY, JSON.stringify(state)); } catch (_) {} }
  function apply() {
    const state = load();
    document.querySelectorAll('.quest-check').forEach(cb => {
      const key = cb.dataset.quest;
      cb.checked = !!state[key];
      cb.addEventListener('change', () => {
        if (cb.checked) state[key] = true; else delete state[key];
        save(state);
      });
    });
  }
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', apply); else apply();
})();
</script>
