<details class="pokemon-card-container">
<summary>Donphan (#054)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-donphan">
<input type="radio" name="pokemon-tabs-donphan-group" id="pokemon-tabs-donphan-tab-0">
<label for="pokemon-tabs-donphan-tab-0">Phanpy</label>
<input type="radio" name="pokemon-tabs-donphan-group" id="pokemon-tabs-donphan-tab-1" checked>
<label for="pokemon-tabs-donphan-tab-1">Donphan</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-donphan-panel-0">
Types: Ground • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Pickup
- Stamina
- Sand Veil *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Poison (0.5×)
- Rock (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Ice (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- HM04 - Strength
- HM06 - Rock Smash

**Encounter Locations**
- Acrisia City — Grass (Day) (10%)
- Acrisia City — Grass (Night) (10%)
- Bronze Fields (North) — Grass (Day) (5%)
- Bronze Fields (North) — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="phanpy" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-mid">345</span> |

**Level-Up Moves**
- Odor Sleuth (Lv 1)
- Tackle (Lv 1)
- Growl (Lv 1)
- Defense Curl (Lv 1)
- Flail (Lv 6)
- Rollout (Lv 10)
- Natural Gift (Lv 13)
- Magnitude (Lv 16)
- Endure (Lv 19)
- Knock Off (Lv 22)
- Slam (Lv 24)
- Take Down (Lv 28)
- Charm (Lv 33)
- Play Rough (Lv 36)
- Last Resort (Lv 37)
- Double-Edge (Lv 42)

**Egg Moves**
- Focus Energy
- Body Slam
- Ancient Power
- Snore
- Counter
- Fissure
- Endeavor
- Ice Shard
- Head Smash
- Mud-Slap
- Heavy Slam
- Play Rough
- High Horsepower

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Endure
- Mud-Slap
- Rock Slide
- Rollout
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
<div class="pokemon-tab-panel" id="pokemon-tabs-donphan-panel-1">
Types: Ground • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sturdy
- Stamina
- Sand Veil *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Poison (0.5×)
- Rock (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Ice (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 25
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="donphan" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">110</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-high">120</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-low">50</span> |
| Total | <span class="stat-value stat-mid">520</span> |

**Level-Up Moves**
- Headbutt (Lv Evo)
- Fire Fang (Lv 1)
- Thunder Fang (Lv 1)
- Horn Attack (Lv 1)
- Bulldoze (Lv 1)
- Growl (Lv 1)
- Defense Curl (Lv 1)
- Rapid Spin (Lv 6)
- Rollout (Lv 10)
- Natural Gift (Lv 13)
- Assurance (Lv 15)
- Magnitude (Lv 16)
- Knock Off (Lv 19)
- Slam (Lv 24)
- Bulldoze (Lv 28)
- Body Slam (Lv 33)
- Play Rough (Lv 36)
- Shore Up (Lv 37)
- Earthquake (Lv 43)
- Giga Impact (Lv 50)

**Egg Moves**
- Focus Energy
- Body Slam
- Ancient Power
- Snore
- Counter
- Fissure
- Endeavor
- Ice Shard
- Head Smash
- Mud-Slap
- Heavy Slam
- Play Rough
- High Horsepower

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Endure
- Mud-Slap
- Rock Slide
- Rollout
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
</div>
</div>
<style>
#pokemon-tabs-donphan-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-donphan-panel-0 { display: block; }
#pokemon-tabs-donphan-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-donphan-panel-1 { display: block; }
</style>
</details>
