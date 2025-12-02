<details class="pokemon-card-container">
<summary>Goodra (#353)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-goodra">
<input type="radio" name="pokemon-tabs-goodra-group" id="pokemon-tabs-goodra-tab-0">
<label for="pokemon-tabs-goodra-tab-0">Goomy</label>
<input type="radio" name="pokemon-tabs-goodra-group" id="pokemon-tabs-goodra-tab-1">
<label for="pokemon-tabs-goodra-tab-1">Sliggoo</label>
<input type="radio" name="pokemon-tabs-goodra-group" id="pokemon-tabs-goodra-tab-2" checked>
<label for="pokemon-tabs-goodra-tab-2">Goodra</label>
<input type="radio" name="pokemon-tabs-goodra-group" id="pokemon-tabs-goodra-tab-3">
<label for="pokemon-tabs-goodra-tab-3">Hisuian Sliggoo</label>
<input type="radio" name="pokemon-tabs-goodra-group" id="pokemon-tabs-goodra-tab-4">
<label for="pokemon-tabs-goodra-tab-4">Hisuian Goodra</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-goodra-panel-0">
Types: Dragon • Egg Groups: Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sap Sipper
- Hydration
- Gooey *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)

*Weak to*
- Ice (2×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM32 - Double Team
- TM34 - Shock Wave
- TM36 - Sludge Bomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract

**Held Item**
Shed Shell

**Encounter Locations**
- Kaptara Island (East) — Grass (Day) (10%)
- Kaptara Island (East) — Grass (Night) (10%)
- Myrrini Island — Grass (Night) (10%)
- Pythios Cemetery — Grass (Day) (5%)
- Pythios Cemetery — Grass (Night) (5%)
- Tower of Dioxippus — Grass (Day) (10%)
- Tower of Dioxippus — Grass (Night) (10%)
- Wanderer's Woods (South) — Grass (Day) (8%)
- Wanderer's Woods (South) — Grass (Night) (8%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="goomy" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-low">35</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-low">300</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Bubble (Lv 1)
- Absorb (Lv 5)
- Protect (Lv 9)
- Bide (Lv 13)
- Dragon Breath (Lv 18)
- Mega Drain (Lv 20)
- Rain Dance (Lv 25)
- Flail (Lv 28)
- Body Slam (Lv 32)
- Muddy Water (Lv 38)
- Dragon Pulse (Lv 42)

**Egg Moves**
- Acid Armor
- Curse
- Iron Tail
- Poison Tail
- Counter
- Endure

**Tutor Moves**
- Acid Spray
- Body Slam
- Counter
- Endure
- Rock Slide
- Sleep Talk
- Snore
- Swagger
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
<div class="pokemon-tab-panel" id="pokemon-tabs-goodra-panel-1">
Types: Dragon • Egg Groups: Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sap Sipper
- Hydration
- Gooey *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)

*Weak to*
- Ice (2×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM32 - Double Team
- TM34 - Shock Wave
- TM36 - Sludge Bomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract

**Held Item**
Shed Shell

**Evolution Info**
Lv. 30
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="sliggoo" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">68</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-mid">53</span> |
| Sp. Atk | <span class="stat-value stat-mid">83</span> |
| Sp. Def | <span class="stat-value stat-high">113</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">452</span> |

**Level-Up Moves**
- Acid Spray (Lv Evo)
- Tackle (Lv 1)
- Bubble (Lv 1)
- Absorb (Lv 5)
- Protect (Lv 9)
- Bide (Lv 13)
- Dragon Breath (Lv 18)
- Mega Drain (Lv 20)
- Rain Dance (Lv 25)
- Flail (Lv 28)
- Body Slam (Lv 32)
- Muddy Water (Lv 38)
- Dragon Pulse (Lv 42)
- Sludge Bomb (Lv 45)
- Power Whip (Lv 50)
- Outrage (Lv 55)

**Egg Moves**
- Acid Armor
- Curse
- Iron Tail
- Poison Tail
- Counter
- Endure

**Tutor Moves**
- Acid Spray
- Body Slam
- Counter
- Endure
- Rock Slide
- Sleep Talk
- Snore
- Swagger
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
<div class="pokemon-tab-panel" id="pokemon-tabs-goodra-panel-2">
Types: Dragon • Egg Groups: Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sap Sipper
- Hydration
- Gooey *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)

*Weak to*
- Ice (2×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM03 - Water Pulse
- TM06 - Toxic
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM32 - Double Team
- TM34 - Shock Wave
- TM35 - Flamethrower
- TM36 - Sludge Bomb
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM56 - Scald
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 50, Overworld Rain
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="goodra" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-high">110</span> |
| Sp. Def | <span class="stat-value stat-high">150</span> |
| Speed | <span class="stat-value stat-mid">80</span> |
| Total | <span class="stat-value stat-high">600</span> |

**Level-Up Moves**
- Aqua Tail (Lv Evo)
- Acid Spray (Lv 1)
- Outrage (Lv 1)
- Feint (Lv 1)
- Tackle (Lv 1)
- Bubble (Lv 1)
- Absorb (Lv 5)
- Protect (Lv 9)
- Bide (Lv 13)
- Dragon Breath (Lv 18)
- Mega Drain (Lv 20)
- Rain Dance (Lv 25)
- Flail (Lv 28)
- Body Slam (Lv 32)
- Muddy Water (Lv 38)
- Dragon Pulse (Lv 42)
- Sludge Bomb (Lv 45)
- Power Whip (Lv 50)
- Outrage (Lv 55)

**Egg Moves**
- Acid Armor
- Curse
- Iron Tail
- Poison Tail
- Counter
- Endure

**Tutor Moves**
- Acid Spray
- Body Slam
- Counter
- Endure
- Fire Punch
- Mega Kick
- Mega Punch
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Thunder Punch
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
<div class="pokemon-tab-panel" id="pokemon-tabs-goodra-panel-3">
Types: Dragon / Steel • Egg Groups: Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sap Sipper
- Shell Armor
- Gooey *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Water (0.5×)
- Electric (0.5×)
- Grass (0.25×)
- Poison (0×)
- Flying (0.5×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Steel (0.5×)

*Weak to*
- Fighting (2×)
- Ground (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM36 - Sludge Bomb
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest

**Held Item**
Shed Shell

**Evolution Info**
Metal Coat?
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-sliggoo" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">58</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-mid">83</span> |
| Sp. Atk | <span class="stat-value stat-mid">83</span> |
| Sp. Def | <span class="stat-value stat-high">113</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-mid">452</span> |

**Level-Up Moves**
- Shelter (Lv Evo)
- Acid Spray (Lv 1)
- Acid Armor (Lv 1)
- Tackle (Lv 1)
- Bubble (Lv 1)
- Absorb (Lv 5)
- Protect (Lv 9)
- Bide (Lv 13)
- Dragon Breath (Lv 18)
- Mega Drain (Lv 20)
- Rain Dance (Lv 25)
- Flail (Lv 28)
- Body Slam (Lv 32)
- Iron Head (Lv 36)
- Dragon Pulse (Lv 42)
- Sludge Bomb (Lv 45)
- Hydro Pump (Lv 50)
- Outrage (Lv 55)

**Egg Moves**
- Acid Armor
- Curse
- Iron Tail
- Poison Tail
- Counter
- Endure

**Tutor Moves**
- Acid Spray
- Body Slam
- Counter
- Endure
- Rock Slide
- Sleep Talk
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
<div class="pokemon-tab-panel" id="pokemon-tabs-goodra-panel-4">
Types: Dragon / Steel • Egg Groups: Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sap Sipper
- Shell Armor
- Gooey *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Water (0.5×)
- Electric (0.5×)
- Grass (0.25×)
- Poison (0×)
- Flying (0.5×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Steel (0.5×)

*Weak to*
- Fighting (2×)
- Ground (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM03 - Water Pulse
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM35 - Flamethrower
- TM36 - Sludge Bomb
- TM37 - Sandstorm
- TM38 - Fire Blast
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM49 - Bulldoze
- HM03 - Surf
- HM06 - Rock Smash

**Evolution Info**
Lv. 50, Overworld Rain?
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-goodra" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-high">110</span> |
| Sp. Def | <span class="stat-value stat-high">150</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-high">600</span> |

**Level-Up Moves**
- Shelter (Lv Evo)
- Acid Spray (Lv 1)
- Acid Armor (Lv 1)
- Tackle (Lv 1)
- Bubble (Lv 1)
- Absorb (Lv 5)
- Protect (Lv 9)
- Bide (Lv 13)
- Dragon Breath (Lv 18)
- Mega Drain (Lv 20)
- Rain Dance (Lv 25)
- Flail (Lv 28)
- Body Slam (Lv 32)
- Iron Head (Lv 36)
- Dragon Pulse (Lv 42)
- Sludge Bomb (Lv 45)
- Hydro Pump (Lv 50)
- Outrage (Lv 55)

**Egg Moves**
- Acid Armor
- Curse
- Iron Tail
- Poison Tail
- Counter
- Endure

**Tutor Moves**
- Acid Spray
- Body Slam
- Counter
- Endure
- Fire Punch
- Rock Slide
- Sleep Talk
- Thunder Punch
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
#pokemon-tabs-goodra-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-goodra-panel-0 { display: block; }
#pokemon-tabs-goodra-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-goodra-panel-1 { display: block; }
#pokemon-tabs-goodra-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-goodra-panel-2 { display: block; }
#pokemon-tabs-goodra-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-goodra-panel-3 { display: block; }
#pokemon-tabs-goodra-tab-4:checked ~ .pokemon-tab-panels #pokemon-tabs-goodra-panel-4 { display: block; }
</style>
</details>
