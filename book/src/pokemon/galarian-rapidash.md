<details class="pokemon-card-container">
<summary>Galarian Rapidash (#253)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-galarian-rapidash">
<input type="radio" name="pokemon-tabs-galarian-rapidash-group" id="pokemon-tabs-galarian-rapidash-tab-0">
<label for="pokemon-tabs-galarian-rapidash-tab-0">Galarian Ponyta</label>
<input type="radio" name="pokemon-tabs-galarian-rapidash-group" id="pokemon-tabs-galarian-rapidash-tab-1" checked>
<label for="pokemon-tabs-galarian-rapidash-tab-1">Galarian Rapidash</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-galarian-rapidash-panel-0">
Types: Psychic • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Run Away
- Pastel Veil
- Anticipation *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Psychic (0.5×)

*Weak to*
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM17 - Protect
- TM29 - Psychic
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam

**Encounter Locations**
- Lastlight Road — Grass (Night) (10%)
- Riverwalk Trail (West) — Grass (Night) (20%)
- Sofos City — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="galarian-ponyta" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">85</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-mid">65</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-mid">410</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Growl (Lv 1)
- Tail Whip (Lv 5)
- Confusion (Lv 10)
- Fairy Wind (Lv 15)
- Agility (Lv 20)
- Psybeam (Lv 25)
- Stomp (Lv 30)
- Heal Pulse (Lv 35)
- Take Down (Lv 41)
- Spirit Break (Lv 45)
- Psychic (Lv 50)
- Healing Wish (Lv 55)

**Egg Moves**
- Thrash
- Double Kick
- Hypnosis
- Double-Edge
- Horn Drill
- Morning Sun

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Sleep Talk
- Snore
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
<div class="pokemon-tab-panel" id="pokemon-tabs-galarian-rapidash-panel-1">
Types: Psychic / Fairy • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Run Away
- Pastel Veil
- Anticipation *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.25×)
- Psychic (0.5×)
- Dragon (0×)

*Weak to*
- Poison (2×)
- Ghost (2×)
- Steel (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM17 - Protect
- TM20 - Poison Jab
- TM29 - Psychic
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam

**Evolution Info**
Lv. 30

**Encounter Locations**
- Port Pello — Grass (Night) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="galarian-rapidash" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-high">115</span> |
| Total | <span class="stat-value stat-mid">510</span> |

**Level-Up Moves**
- Psycho Cut (Lv Evo)
- Megahorn (Lv 1)
- Tackle (Lv 1)
- Growl (Lv 1)
- Tail Whip (Lv 5)
- Confusion (Lv 10)
- Fairy Wind (Lv 15)
- Agility (Lv 20)
- Psybeam (Lv 25)
- Stomp (Lv 30)
- Heal Pulse (Lv 35)
- Take Down (Lv 41)
- Spirit Break (Lv 45)
- Psychic (Lv 50)
- Healing Wish (Lv 55)

**Egg Moves**
- Thrash
- Double Kick
- Hypnosis
- Double-Edge
- Horn Drill
- Morning Sun

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Sleep Talk
- Snore
- Swift
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
</div>
</div>
<style>
#pokemon-tabs-galarian-rapidash-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-galarian-rapidash-panel-0 { display: block; }
#pokemon-tabs-galarian-rapidash-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-galarian-rapidash-panel-1 { display: block; }
</style>
</details>
