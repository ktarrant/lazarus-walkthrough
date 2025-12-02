<details class="pokemon-card-container">
<summary>Hisuian Growlithe (#376)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-hisuian-growlithe">
<input type="radio" name="pokemon-tabs-hisuian-growlithe-group" id="pokemon-tabs-hisuian-growlithe-tab-0" checked>
<label for="pokemon-tabs-hisuian-growlithe-tab-0">Hisuian Growlithe</label>
<input type="radio" name="pokemon-tabs-hisuian-growlithe-group" id="pokemon-tabs-hisuian-growlithe-tab-1">
<label for="pokemon-tabs-hisuian-growlithe-tab-1">Hisuian Arcanine</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-hisuian-growlithe-panel-0">
Types: Fire / Rock • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Rock Head
- Fur Coat *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.25×)
- Ice (0.5×)
- Poison (0.5×)
- Flying (0.5×)
- Bug (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (4×)
- Fighting (2×)
- Ground (4×)
- Rock (2×)

**TM/HM Moves**
- TM11 - Sunny Day
- TM17 - Protect
- TM28 - Dig
- TM35 - Flamethrower
- TM37 - Sandstorm
- TM38 - Fire Blast
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM51 - Will-O-Wisp
- TM55 - Snarl
- HM06 - Rock Smash

**Encounter Locations**
- Jusmail Town — Grass (Day) (4%)
- Marmaro Island — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-growlithe" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-mid">65</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-mid">55</span> |
| Total | <span class="stat-value stat-mid">350</span> |

**Level-Up Moves**
- Bite (Lv 1)
- Roar (Lv 1)
- Ember (Lv 6)
- Leer (Lv 8)
- Rock Throw (Lv 10)
- Helping Hand (Lv 12)
- Flame Wheel (Lv 17)
- Reversal (Lv 19)
- Fire Fang (Lv 21)
- Take Down (Lv 23)
- Flame Burst (Lv 28)
- Smack Down (Lv 30)
- Raging Fury (Lv 32)
- Flamethrower (Lv 34)
- Crunch (Lv 37)
- Rock Slide (Lv 40)
- Outrage (Lv 43)
- Flare Blitz (Lv 45)
- Head Smash (Lv 50)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Rock Slide
- Sleep Talk
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
<div class="pokemon-tab-panel" id="pokemon-tabs-hisuian-growlithe-panel-1">
Types: Fire / Rock • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Rock Head
- Fur Coat *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.25×)
- Ice (0.5×)
- Poison (0.5×)
- Flying (0.5×)
- Bug (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (4×)
- Fighting (2×)
- Ground (4×)
- Rock (2×)

**TM/HM Moves**
- TM11 - Sunny Day
- TM17 - Protect
- TM22 - Solar Beam
- TM28 - Dig
- TM35 - Flamethrower
- TM37 - Sandstorm
- TM38 - Fire Blast
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM46 - Thief
- TM49 - Bulldoze
- TM51 - Will-O-Wisp
- TM55 - Snarl
- HM06 - Rock Smash

**Evolution Info**
Fire Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-arcanine" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">115</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-high">95</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-high">555</span> |

**Level-Up Moves**
- Accelerock (Lv Evo)
- Bite (Lv 1)
- Roar (Lv 1)
- Ember (Lv 6)
- Leer (Lv 8)
- Rock Throw (Lv 10)
- Helping Hand (Lv 12)
- Flame Wheel (Lv 17)
- Reversal (Lv 19)
- Fire Fang (Lv 21)
- Take Down (Lv 23)
- Flame Burst (Lv 28)
- Smack Down (Lv 30)
- Raging Fury (Lv 32)
- Flamethrower (Lv 34)
- Crunch (Lv 37)
- Rock Slide (Lv 40)
- Outrage (Lv 43)
- Flare Blitz (Lv 45)
- Head Smash (Lv 50)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Rock Slide
- Sleep Talk
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
#pokemon-tabs-hisuian-growlithe-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-hisuian-growlithe-panel-0 { display: block; }
#pokemon-tabs-hisuian-growlithe-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-hisuian-growlithe-panel-1 { display: block; }
</style>
</details>
