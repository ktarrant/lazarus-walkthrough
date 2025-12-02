<details class="pokemon-card-container">
<summary>Litten (#004)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-litten">
<input type="radio" name="pokemon-tabs-litten-group" id="pokemon-tabs-litten-tab-0" checked>
<label for="pokemon-tabs-litten-tab-0">Litten</label>
<input type="radio" name="pokemon-tabs-litten-group" id="pokemon-tabs-litten-tab-1">
<label for="pokemon-tabs-litten-tab-1">Torracat</label>
<input type="radio" name="pokemon-tabs-litten-group" id="pokemon-tabs-litten-tab-2">
<label for="pokemon-tabs-litten-tab-2">Incineroar</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-litten-panel-0">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Rivalry
- Intimidate *(Hidden)*

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
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM32 - Double Team
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM51 - Will-O-Wisp

**Encounter Locations**
- Sea of Vulcai — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="litten" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-mid">320</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Ember (Lv 1)
- Growl (Lv 4)
- Lick (Lv 8)
- Leer (Lv 11)
- Fire Fang (Lv 14)
- Double Kick (Lv 16)
- Roar (Lv 18)
- Bite (Lv 22)
- Swagger (Lv 25)
- Fury Swipes (Lv 29)
- Thrash (Lv 32)
- Flamethrower (Lv 35)
- Parting Shot (Lv 39)
- Flare Blitz (Lv 43)
- Outrage (Lv 46)

**Egg Moves**
- Nasty Plot
- Body Slam
- Crunch
- Fake Out
- Revenge
- Heat Wave
- Power Trip

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Sleep Talk
- Snore
- Swagger
- Swords Dance
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
<div class="pokemon-tab-panel" id="pokemon-tabs-litten-panel-1">
Types: Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Rivalry
- Intimidate *(Hidden)*

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
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM32 - Double Team
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM51 - Will-O-Wisp

**Evolution Info**
Lv. 17
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="torracat" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-mid">85</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-mid">420</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Ember (Lv 1)
- Growl (Lv 4)
- Lick (Lv 8)
- Leer (Lv 11)
- Fire Fang (Lv 14)
- Double Kick (Lv 16)
- Roar (Lv 18)
- Bite (Lv 22)
- Swagger (Lv 25)
- Fury Swipes (Lv 29)
- Thrash (Lv 32)
- Flamethrower (Lv 35)
- Parting Shot (Lv 39)
- Crush Claw (Lv 42)
- Flare Blitz (Lv 46)
- Outrage (Lv 50)

**Egg Moves**
- Nasty Plot
- Body Slam
- Crunch
- Fake Out
- Revenge
- Heat Wave
- Power Trip

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Sleep Talk
- Snore
- Swagger
- Swords Dance
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
<div class="pokemon-tab-panel" id="pokemon-tabs-litten-panel-2">
Types: Fire / Dark • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Blaze
- Rivalry
- Intimidate *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)
- Steel (0.5×)

*Weak to*
- Water (2×)
- Fighting (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM26 - Earthquake
- TM31 - Brick Break
- TM32 - Double Team
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM51 - Will-O-Wisp
- TM55 - Snarl
- TM59 - Dark Pulse

**Evolution Info**
Lv. 34
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="incineroar" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">115</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- Darkest Lariat (Lv Evo)
- Bulk Up (Lv 1)
- Throat Chop (Lv 1)
- Scratch (Lv 1)
- Ember (Lv 1)
- Growl (Lv 4)
- Lick (Lv 8)
- Leer (Lv 11)
- Fire Fang (Lv 14)
- Double Kick (Lv 16)
- Roar (Lv 18)
- Bite (Lv 22)
- Swagger (Lv 25)
- Fury Swipes (Lv 29)
- Thrash (Lv 32)
- Flamethrower (Lv 35)
- Parting Shot (Lv 39)
- Crush Claw (Lv 42)
- Flare Blitz (Lv 46)
- Outrage (Lv 50)
- Cross Chop (Lv 55)

**Egg Moves**
- Nasty Plot
- Body Slam
- Crunch
- Fake Out
- Revenge
- Heat Wave
- Power Trip

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Fire Punch
- Mega Kick
- Mega Punch
- Sleep Talk
- Snore
- Swagger
- Swords Dance
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
#pokemon-tabs-litten-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-litten-panel-0 { display: block; }
#pokemon-tabs-litten-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-litten-panel-1 { display: block; }
#pokemon-tabs-litten-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-litten-panel-2 { display: block; }
</style>
</details>
