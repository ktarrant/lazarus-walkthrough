<details class="pokemon-card-container">
<summary>Archen (#406)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-archen">
<input type="radio" name="pokemon-tabs-archen-group" id="pokemon-tabs-archen-tab-0" checked>
<label for="pokemon-tabs-archen-tab-0">Archen</label>
<input type="radio" name="pokemon-tabs-archen-group" id="pokemon-tabs-archen-tab-1">
<label for="pokemon-tabs-archen-tab-1">Archeops</label>
<input type="radio" name="pokemon-tabs-archen-group" id="pokemon-tabs-archen-tab-2">
<label for="pokemon-tabs-archen-tab-2">Mega Archeops</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-archen-panel-0">
Types: Rock / Flying • Egg Groups: Water 3 / Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Defeatist
- Tangled Feet *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.5×)
- Poison (0.5×)
- Ground (0×)
- Flying (0.5×)
- Bug (0.5×)

*Weak to*
- Water (2×)
- Electric (2×)
- Ice (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM12 - Taunt
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM47 - Steel Wing
- TM49 - Bulldoze
- TM57 - Roost
- HM01 - Cut
- HM06 - Rock Smash

**Encounter Locations**
- Palati City — Grass (Day) (10%)
- Palati City — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="archen" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">55</span> |
| Attack | <span class="stat-value stat-high">112</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-mid">74</span> |
| Sp. Def | <span class="stat-value stat-low">45</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-mid">401</span> |

**Level-Up Moves**
- Quick Attack (Lv 1)
- Leer (Lv 1)
- Wing Attack (Lv 1)
- Rock Throw (Lv 5)
- Double Team (Lv 8)
- Bite (Lv 11)
- Pluck (Lv 15)
- Ancient Power (Lv 18)
- Agility (Lv 21)
- Quick Guard (Lv 25)
- Acrobatics (Lv 28)
- Dragon Breath (Lv 31)
- Crunch (Lv 35)
- Endeavor (Lv 38)
- U-Turn (Lv 40)
- Knock Off (Lv 42)
- Rock Slide (Lv 45)
- Dragon Claw (Lv 48)
- Thrash (Lv 50)

**Egg Moves**
- Steel Wing
- Defog
- Dragon Pulse
- Head Smash
- Knock Off
- Earth Power
- Bite
- Ally Switch
- Switcheroo

**Tutor Moves**
- Endure
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
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
<div class="pokemon-tab-panel" id="pokemon-tabs-archen-panel-1">
Types: Rock / Flying • Egg Groups: Water 3 / Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Defeatist
- Fluffy *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.5×)
- Poison (0.5×)
- Ground (0×)
- Flying (0.5×)
- Bug (0.5×)

*Weak to*
- Water (2×)
- Electric (2×)
- Ice (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM12 - Taunt
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM47 - Steel Wing
- TM49 - Bulldoze
- TM57 - Roost
- HM01 - Cut
- HM02 - Fly
- HM06 - Rock Smash

**Evolution Info**
Lv. 33
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="archeops" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-high">125</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-high">100</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-high">110</span> |
| Total | <span class="stat-value stat-high">550</span> |

**Level-Up Moves**
- Crunch (Lv Evo)
- Iron Head (Lv 1)
- Quick Attack (Lv 1)
- Leer (Lv 1)
- Wing Attack (Lv 1)
- Rock Throw (Lv 5)
- Double Team (Lv 8)
- Bite (Lv 11)
- Pluck (Lv 15)
- Ancient Power (Lv 18)
- Agility (Lv 21)
- Quick Guard (Lv 25)
- Acrobatics (Lv 28)
- Dragon Breath (Lv 31)
- Pounce (Lv 35)
- Endeavor (Lv 38)
- U-Turn (Lv 40)
- Knock Off (Lv 42)
- Rock Slide (Lv 45)
- Dragon Claw (Lv 48)
- Thrash (Lv 50)
- Brave Bird (Lv 55)

**Egg Moves**
- Steel Wing
- Defog
- Dragon Pulse
- Head Smash
- Knock Off
- Earth Power
- Bite
- Ally Switch
- Switcheroo

**Tutor Moves**
- Endure
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
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
<div class="pokemon-tab-panel" id="pokemon-tabs-archen-panel-2">
Types: Rock / Flying • Egg Groups: Water 3 / Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.5×)
- Poison (0.5×)
- Ground (0×)
- Flying (0.5×)
- Bug (0.5×)

*Weak to*
- Water (2×)
- Electric (2×)
- Ice (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM12 - Taunt
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM47 - Steel Wing
- TM49 - Bulldoze
- TM57 - Roost
- HM01 - Cut
- HM02 - Fly
- HM06 - Rock Smash

**Evolution Info**
Archeopsite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-archeops" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-high">165</span> |
| Defense | <span class="stat-value stat-mid">85</span> |
| Sp. Atk | <span class="stat-value stat-high">110</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-high">140</span> |
| Total | <span class="stat-value stat-high">650</span> |

**Level-Up Moves**
- Crunch (Lv Evo)
- Iron Head (Lv 1)
- Quick Attack (Lv 1)
- Leer (Lv 1)
- Wing Attack (Lv 1)
- Rock Throw (Lv 5)
- Double Team (Lv 8)
- Bite (Lv 11)
- Pluck (Lv 15)
- Ancient Power (Lv 18)
- Agility (Lv 21)
- Quick Guard (Lv 25)
- Acrobatics (Lv 28)
- Dragon Breath (Lv 31)
- Pounce (Lv 35)
- Endeavor (Lv 38)
- U-Turn (Lv 40)
- Knock Off (Lv 42)
- Rock Slide (Lv 45)
- Dragon Claw (Lv 48)
- Thrash (Lv 50)
- Brave Bird (Lv 55)

**Egg Moves**
- Steel Wing
- Defog
- Dragon Pulse
- Head Smash
- Knock Off
- Earth Power
- Bite
- Ally Switch
- Switcheroo

**Tutor Moves**
- Endure
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
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
#pokemon-tabs-archen-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-archen-panel-0 { display: block; }
#pokemon-tabs-archen-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-archen-panel-1 { display: block; }
#pokemon-tabs-archen-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-archen-panel-2 { display: block; }
</style>
</details>
