<details class="pokemon-card-container">
<summary>Gyarados (#343)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-gyarados">
<input type="radio" name="pokemon-tabs-gyarados-group" id="pokemon-tabs-gyarados-tab-0">
<label for="pokemon-tabs-gyarados-tab-0">Magikarp</label>
<input type="radio" name="pokemon-tabs-gyarados-group" id="pokemon-tabs-gyarados-tab-1" checked>
<label for="pokemon-tabs-gyarados-tab-1">Gyarados</label>
<input type="radio" name="pokemon-tabs-gyarados-group" id="pokemon-tabs-gyarados-tab-2">
<label for="pokemon-tabs-gyarados-tab-2">Mega Gyarados</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-gyarados-panel-0">
Types: Water • Egg Groups: Water 2 / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Swift Swim
- Rattled *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Encounter Locations**
- Acrisia City — Fishing (70%)
- Acrisia Mountains — Fishing (70%)
- Corrin Crossing — Fishing (70%)
- Davosi Straits — Fishing (70%)
- Erinys Path (East) — Fishing (70%)
- Fresco Isles — Fishing (70%)
- Froslass Cavern BF2 — Fishing (70%)
- Kalami City — Fishing (70%)
- Marmaro Island — Fishing (70%)
- Myrrini Island — Fishing (70%)
- Nyx Trails — Fishing (70%)
- Palati City — Fishing (70%)
- Pollen Road — Fishing (70%)
- Port Pello — Fishing (70%)
- Pythios Cemetery — Fishing (70%)
- Pythios Town — Fishing (70%)
- Péntepetal City — Fishing (70%)
- Riverwalk Trail (South) — Fishing (70%)
- Riverwalk Trail (West) — Fishing (70%)
- Sea of Asteri (East) — Fishing (70%)
- Sea of Asteri (West) — Fishing (70%)
- Sea of Asteri (West) — Dive (10%)
- Sea of Vulcai — Fishing (70%)
- Sofos City — Fishing (70%)
- Wanderer's Woods (North) — Fishing (70%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="magikarp" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">20</span> |
| Attack | <span class="stat-value stat-low">10</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-low">15</span> |
| Sp. Def | <span class="stat-value stat-low">20</span> |
| Speed | <span class="stat-value stat-mid">80</span> |
| Total | <span class="stat-value stat-low">200</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=splash">Splash</a> (Lv 1)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 15)
- <a href="move-lookup.html?q=flail">Flail</a> (Lv 30)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-gyarados-panel-1">
Types: Water / Flying • Egg Groups: Water 2 / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Moxie *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Fighting (0.5×)
- Ground (0×)
- Bug (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (4×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=water-pulse">TM03 - Water Pulse</a>
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=whirlpool">TM07 - Whirlpool</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=blizzard">TM14 - Blizzard</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=deepwater-curse">TM50 - Deepwater Curse</a>
- <a href="move-lookup.html?q=scald">TM56 - Scald</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=dark-pulse">TM59 - Dark Pulse</a>
- <a href="move-lookup.html?q=dragon-dance">TM60 - Dragon Dance</a>
- <a href="move-lookup.html?q=surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=strength">HM04 - Strength</a>
- <a href="move-lookup.html?q=rock-smash">HM06 - Rock Smash</a>
- <a href="move-lookup.html?q=waterfall">HM07 - Waterfall</a>
- <a href="move-lookup.html?q=dive">HM08 - Dive</a>

**Evolution Info**
Lv. 20
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="gyarados" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">125</span> |
| Defense | <span class="stat-value stat-mid">79</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-high">100</span> |
| Speed | <span class="stat-value stat-mid">81</span> |
| Total | <span class="stat-value stat-mid">540</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=bite">Bite</a> (Lv Evo)
- <a href="move-lookup.html?q=poison-fang">Poison Fang</a> (Lv 1)
- <a href="move-lookup.html?q=deepwater-curse">Deepwater Curse</a> (Lv 1)
- <a href="move-lookup.html?q=thrash">Thrash</a> (Lv 1)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 21)
- <a href="move-lookup.html?q=twister">Twister</a> (Lv 24)
- <a href="move-lookup.html?q=ice-fang">Ice Fang</a> (Lv 27)
- <a href="move-lookup.html?q=aqua-tail">Aqua Tail</a> (Lv 30)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 33)
- <a href="move-lookup.html?q=dragon-rage">Dragon Rage</a> (Lv 36)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 39)
- <a href="move-lookup.html?q=hydro-pump">Hydro Pump</a> (Lv 42)
- <a href="move-lookup.html?q=dragon-dance">Dragon Dance</a> (Lv 45)
- <a href="move-lookup.html?q=hurricane">Hurricane</a> (Lv 48)
- <a href="move-lookup.html?q=rain-dance">Rain Dance</a> (Lv 51)
- <a href="move-lookup.html?q=hyper-beam">Hyper Beam</a> (Lv 54)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-gyarados-panel-2">
Types: Water / Dark • Egg Groups: Water 2 / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Mold Breaker

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Fighting (2×)
- Bug (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=water-pulse">TM03 - Water Pulse</a>
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=whirlpool">TM07 - Whirlpool</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=blizzard">TM14 - Blizzard</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=deepwater-curse">TM50 - Deepwater Curse</a>
- <a href="move-lookup.html?q=scald">TM56 - Scald</a>
- <a href="move-lookup.html?q=thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=dark-pulse">TM59 - Dark Pulse</a>
- <a href="move-lookup.html?q=dragon-dance">TM60 - Dragon Dance</a>
- <a href="move-lookup.html?q=surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=strength">HM04 - Strength</a>
- <a href="move-lookup.html?q=rock-smash">HM06 - Rock Smash</a>
- <a href="move-lookup.html?q=waterfall">HM07 - Waterfall</a>
- <a href="move-lookup.html?q=dive">HM08 - Dive</a>

**Evolution Info**
Gyaradosite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-gyarados" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">155</span> |
| Defense | <span class="stat-value stat-high">109</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-high">130</span> |
| Speed | <span class="stat-value stat-mid">81</span> |
| Total | <span class="stat-value stat-high">640</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=bite">Bite</a> (Lv Evo)
- <a href="move-lookup.html?q=poison-fang">Poison Fang</a> (Lv 1)
- <a href="move-lookup.html?q=deepwater-curse">Deepwater Curse</a> (Lv 1)
- <a href="move-lookup.html?q=thrash">Thrash</a> (Lv 1)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 21)
- <a href="move-lookup.html?q=twister">Twister</a> (Lv 24)
- <a href="move-lookup.html?q=ice-fang">Ice Fang</a> (Lv 27)
- <a href="move-lookup.html?q=aqua-tail">Aqua Tail</a> (Lv 30)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 33)
- <a href="move-lookup.html?q=dragon-rage">Dragon Rage</a> (Lv 36)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 39)
- <a href="move-lookup.html?q=hydro-pump">Hydro Pump</a> (Lv 42)
- <a href="move-lookup.html?q=dragon-dance">Dragon Dance</a> (Lv 45)
- <a href="move-lookup.html?q=hurricane">Hurricane</a> (Lv 48)
- <a href="move-lookup.html?q=rain-dance">Rain Dance</a> (Lv 51)
- <a href="move-lookup.html?q=hyper-beam">Hyper Beam</a> (Lv 54)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
</div>
</div>
<style>
#pokemon-tabs-gyarados-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-gyarados-panel-0 { display: block; }
#pokemon-tabs-gyarados-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-gyarados-panel-1 { display: block; }
#pokemon-tabs-gyarados-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-gyarados-panel-2 { display: block; }
</style>
</details>
