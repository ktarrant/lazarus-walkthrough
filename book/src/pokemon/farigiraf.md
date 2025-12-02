<details class="pokemon-card-container">
<summary>Farigiraf (#367)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-farigiraf">
<input type="radio" name="pokemon-tabs-farigiraf-group" id="pokemon-tabs-farigiraf-tab-0">
<label for="pokemon-tabs-farigiraf-tab-0">Girafarig</label>
<input type="radio" name="pokemon-tabs-farigiraf-group" id="pokemon-tabs-farigiraf-tab-1" checked>
<label for="pokemon-tabs-farigiraf-tab-1">Farigiraf</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-farigiraf-panel-0">
Types: Normal / Psychic • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Inner Focus
- Early Bird
- Sap Sipper *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Psychic (0.5×)
- Ghost (0×)

*Weak to*
- Bug (2×)
- Dark (2×)

**TM/HM Moves**
- TM01 - Wish
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM49 - Bulldoze
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- HM04 - Strength
- HM05 - Flash
- HM06 - Rock Smash

**Encounter Locations**
- Corrin Crossing — Grass (Night) (10%)
- Palati City — Grass (Day) (10%)
- Palati City — Grass (Night) (10%)
- Sofos City — Grass (Day) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="girafarig" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-mid">90</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-mid">85</span> |
| Total | <span class="stat-value stat-mid">455</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Growl (Lv 1)
- Astonish (Lv 1)
- Power Swap (Lv 1)
- Guard Swap (Lv 1)
- Confusion (Lv 5)
- Assurance (Lv 10)
- Stomp (Lv 14)
- Psybeam (Lv 19)
- Agility (Lv 23)
- Double Hit (Lv 28)
- Twin Beam (Lv 32)
- Crunch (Lv 37)
- Baton Pass (Lv 41)
- Nasty Plot (Lv 46)
- Psychic (Lv 50)

**Egg Moves**
- Take Down
- Amnesia
- Foresight
- Future Sight
- Beat Up
- Wish
- Magic Coat
- Double Kick
- Mirror Coat
- Razor Wind
- Skill Swap
- Secret Power
- Mean Look
- Psychic Terrain
- Psychic Fangs

**Tutor Moves**
- Body Slam
- Double-Edge
- Dream Eater
- Endure
- Mud-Slap
- Psych Up
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
<div class="pokemon-tab-panel" id="pokemon-tabs-farigiraf-panel-1">
Types: Normal / Psychic • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Cud Chew
- Armor Tail
- Sap Sipper *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Psychic (0.5×)
- Ghost (0×)

*Weak to*
- Bug (2×)
- Dark (2×)

**TM/HM Moves**
- TM01 - Wish
- TM04 - Calm Mind
- TM05 - Psyshock
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM29 - Psychic
- TM30 - Shadow Ball
- TM33 - Reflect
- TM42 - Facade
- TM44 - Rest
- TM46 - Thief
- TM48 - Skill Swap
- TM49 - Bulldoze
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave

**Evolution Info**
Lv. knows Twin Beam

**Encounter Locations**
- Corrin Crossing — Grass (Night) (5%)
- Wakewater Isle — Grass (Day) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="farigiraf" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">120</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-high">110</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">520</span> |

**Level-Up Moves**
- Tri Attack (Lv Evo)
- Tackle (Lv 1)
- Growl (Lv 1)
- Astonish (Lv 1)
- Power Swap (Lv 1)
- Guard Swap (Lv 1)
- Confusion (Lv 5)
- Assurance (Lv 10)
- Stomp (Lv 14)
- Psybeam (Lv 19)
- Agility (Lv 23)
- Double Hit (Lv 28)
- Twin Beam (Lv 32)
- Crunch (Lv 37)
- Baton Pass (Lv 41)
- Nasty Plot (Lv 46)
- Psychic (Lv 50)
- Hyper Voice (Lv 53)

**Egg Moves**
- Take Down
- Amnesia
- Foresight
- Future Sight
- Beat Up
- Wish
- Magic Coat
- Double Kick
- Mirror Coat
- Razor Wind
- Skill Swap
- Secret Power
- Mean Look
- Psychic Terrain
- Psychic Fangs

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Psych Up
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
#pokemon-tabs-farigiraf-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-farigiraf-panel-0 { display: block; }
#pokemon-tabs-farigiraf-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-farigiraf-panel-1 { display: block; }
</style>
</details>
