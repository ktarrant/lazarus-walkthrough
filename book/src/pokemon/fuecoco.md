<details class="pokemon-card-container">
<summary>Fuecoco (#022)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-fuecoco">
<input type="radio" name="pokemon-tabs-fuecoco-group" id="pokemon-tabs-fuecoco-tab-0" checked>
<label for="pokemon-tabs-fuecoco-tab-0">Fuecoco</label>
<input type="radio" name="pokemon-tabs-fuecoco-group" id="pokemon-tabs-fuecoco-tab-1">
<label for="pokemon-tabs-fuecoco-tab-1">Crocalor</label>
<input type="radio" name="pokemon-tabs-fuecoco-group" id="pokemon-tabs-fuecoco-tab-2">
<label for="pokemon-tabs-fuecoco-tab-2">Skeledirge</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-fuecoco-panel-0">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Unaware
- Dry Skin *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Bug (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm35-flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=tm38-fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm51-will-o-wisp">TM51 - Will-O-Wisp</a>
- <a href="move-lookup.html?q=tm55-snarl">TM55 - Snarl</a>

**Encounter Locations**
- Sea of Vulcai — Grass (Day) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="fuecoco" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">67</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-mid">59</span> |
| Sp. Atk | <span class="stat-value stat-mid">63</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-low">36</span> |
| Total | <span class="stat-value stat-mid">310</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 1)
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=round">Round</a> (Lv 7)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 12)
- <a href="move-lookup.html?q=incinerate">Incinerate</a> (Lv 15)
- <a href="move-lookup.html?q=yawn">Yawn</a> (Lv 17)
- <a href="move-lookup.html?q=snarl">Snarl</a> (Lv 21)
- <a href="move-lookup.html?q=roar">Roar</a> (Lv 25)
- <a href="move-lookup.html?q=flamethrower">Flamethrower</a> (Lv 28)
- <a href="move-lookup.html?q=hyper-voice">Hyper Voice</a> (Lv 32)
- <a href="move-lookup.html?q=fire-blast">Fire Blast</a> (Lv 36)

**Egg Moves**
- <a href="move-lookup.html?q=belch">Belch</a>
- <a href="move-lookup.html?q=curse">Curse</a>
- <a href="move-lookup.html?q=encore">Encore</a>
- <a href="move-lookup.html?q=slack-off">Slack Off</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-fuecoco-panel-1">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Unaware
- Dry Skin *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Bug (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm35-flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=tm38-fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm51-will-o-wisp">TM51 - Will-O-Wisp</a>
- <a href="move-lookup.html?q=tm55-snarl">TM55 - Snarl</a>

**Evolution Info**
Lv. 16
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="crocalor" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">81</span> |
| Attack | <span class="stat-value stat-mid">55</span> |
| Defense | <span class="stat-value stat-mid">78</span> |
| Sp. Atk | <span class="stat-value stat-mid">90</span> |
| Sp. Def | <span class="stat-value stat-mid">58</span> |
| Speed | <span class="stat-value stat-low">49</span> |
| Total | <span class="stat-value stat-mid">411</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 1)
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=lick">Lick</a> (Lv 7)
- <a href="move-lookup.html?q=round">Round</a> (Lv 10)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 12)
- <a href="move-lookup.html?q=yawn">Yawn</a> (Lv 15)
- <a href="move-lookup.html?q=incinerate">Incinerate</a> (Lv 17)
- <a href="move-lookup.html?q=snarl">Snarl</a> (Lv 24)
- <a href="move-lookup.html?q=roar">Roar</a> (Lv 28)
- <a href="move-lookup.html?q=flamethrower">Flamethrower</a> (Lv 32)
- <a href="move-lookup.html?q=hyper-voice">Hyper Voice</a> (Lv 38)
- <a href="move-lookup.html?q=will-o-wisp">Will-O-Wisp</a> (Lv 42)
- <a href="move-lookup.html?q=fire-blast">Fire Blast</a> (Lv 47)

**Egg Moves**
- <a href="move-lookup.html?q=belch">Belch</a>
- <a href="move-lookup.html?q=curse">Curse</a>
- <a href="move-lookup.html?q=encore">Encore</a>
- <a href="move-lookup.html?q=slack-off">Slack Off</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-fuecoco-panel-2">
Types: Fire / Ghost • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Unaware
- Dry Skin *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.25×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm23-hex">TM23 - Hex</a>
- <a href="move-lookup.html?q=tm26-earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm30-shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=tm35-flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=tm38-fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm51-will-o-wisp">TM51 - Will-O-Wisp</a>
- <a href="move-lookup.html?q=tm55-snarl">TM55 - Snarl</a>

**Evolution Info**
Lv. 36
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="skeledirge" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">104</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-high">110</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-mid">66</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=torch-song">Torch Song</a> (Lv Evo)
- <a href="move-lookup.html?q=sing">Sing</a> (Lv 1)
- <a href="move-lookup.html?q=yawn">Yawn</a> (Lv 1)
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 1)
- <a href="move-lookup.html?q=lick">Lick</a> (Lv 7)
- <a href="move-lookup.html?q=round">Round</a> (Lv 10)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 12)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 15)
- <a href="move-lookup.html?q=incinerate">Incinerate</a> (Lv 17)
- <a href="move-lookup.html?q=snarl">Snarl</a> (Lv 24)
- <a href="move-lookup.html?q=roar">Roar</a> (Lv 28)
- <a href="move-lookup.html?q=flamethrower">Flamethrower</a> (Lv 32)
- <a href="move-lookup.html?q=shadow-ball">Shadow Ball</a> (Lv 38)
- <a href="move-lookup.html?q=hyper-voice">Hyper Voice</a> (Lv 40)
- <a href="move-lookup.html?q=will-o-wisp">Will-O-Wisp</a> (Lv 43)
- <a href="move-lookup.html?q=hex">Hex</a> (Lv 46)
- <a href="move-lookup.html?q=fire-blast">Fire Blast</a> (Lv 50)
- <a href="move-lookup.html?q=overheat">Overheat</a> (Lv 55)

**Egg Moves**
- <a href="move-lookup.html?q=belch">Belch</a>
- <a href="move-lookup.html?q=curse">Curse</a>
- <a href="move-lookup.html?q=encore">Encore</a>
- <a href="move-lookup.html?q=slack-off">Slack Off</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
#pokemon-tabs-fuecoco-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-fuecoco-panel-0 { display: block; }
#pokemon-tabs-fuecoco-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-fuecoco-panel-1 { display: block; }
#pokemon-tabs-fuecoco-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-fuecoco-panel-2 { display: block; }
</style>
</details>
