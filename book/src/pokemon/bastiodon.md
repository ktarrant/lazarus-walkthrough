<details class="pokemon-card-container">
<summary>Bastiodon (#229)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-bastiodon">
<input type="radio" name="pokemon-tabs-bastiodon-group" id="pokemon-tabs-bastiodon-tab-0">
<label for="pokemon-tabs-bastiodon-tab-0">Shieldon</label>
<input type="radio" name="pokemon-tabs-bastiodon-group" id="pokemon-tabs-bastiodon-tab-1" checked>
<label for="pokemon-tabs-bastiodon-tab-1">Bastiodon</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-bastiodon-panel-0">
Types: Rock / Steel • Egg Groups: Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sturdy
- Rock Head
- Soundproof *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.25×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.25×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dragon (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Fighting (4×)
- Ground (4×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM34 - Shock Wave
- TM35 - Flamethrower
- TM37 - Sandstorm
- TM38 - Fire Blast
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM04 - Strength
- HM06 - Rock Smash
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="shieldon" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-low">47</span> |
| Defense | <span class="stat-value stat-high">118</span> |
| Sp. Atk | <span class="stat-value stat-low">37</span> |
| Sp. Def | <span class="stat-value stat-mid">88</span> |
| Speed | <span class="stat-value stat-low">30</span> |
| Total | <span class="stat-value stat-mid">360</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Protect (Lv 1)
- Taunt (Lv 6)
- Metal Sound (Lv 10)
- Take Down (Lv 14)
- Magnet Bomb (Lv 17)
- Iron Defense (Lv 20)
- Rock Throw (Lv 22)
- Swagger (Lv 24)
- Endure (Lv 26)
- Ancient Power (Lv 30)
- Slack Off (Lv 34)
- Metal Burst (Lv 37)
- Iron Head (Lv 42)
- Heavy Slam (Lv 46)
- Body Press (Lv 50)

**Egg Moves**
- Headbutt
- Scary Face
- Focus Energy
- Double-Edge
- Rock Blast
- Body Slam
- Screech
- Curse
- Fissure
- Counter
- Stealth Rock
- Wide Guard
- Guard Split

**Tutor Moves**
- Body Slam
- Counter
- Double-Edge
- Endure
- Mud-Slap
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
<div class="pokemon-tab-panel" id="pokemon-tabs-bastiodon-panel-1">
Types: Rock / Steel • Egg Groups: Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sturdy
- Rock Head
- Soundproof *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.25×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.25×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dragon (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Fighting (4×)
- Ground (4×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM35 - Flamethrower
- TM37 - Sandstorm
- TM38 - Fire Blast
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 30
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="bastiodon" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-mid">67</span> |
| Defense | <span class="stat-value stat-high">168</span> |
| Sp. Atk | <span class="stat-value stat-low">37</span> |
| Sp. Def | <span class="stat-value stat-high">128</span> |
| Speed | <span class="stat-value stat-low">30</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Block (Lv Evo)
- Tackle (Lv 1)
- Protect (Lv 1)
- Taunt (Lv 6)
- Metal Sound (Lv 10)
- Take Down (Lv 14)
- Magnet Bomb (Lv 17)
- Iron Defense (Lv 20)
- Rock Throw (Lv 22)
- Swagger (Lv 24)
- Endure (Lv 26)
- Ancient Power (Lv 30)
- Slack Off (Lv 34)
- Metal Burst (Lv 37)
- Iron Head (Lv 42)
- Heavy Slam (Lv 46)
- Body Press (Lv 50)

**Egg Moves**
- Headbutt
- Scary Face
- Focus Energy
- Double-Edge
- Rock Blast
- Body Slam
- Screech
- Curse
- Fissure
- Counter
- Stealth Rock
- Wide Guard
- Guard Split

**Tutor Moves**
- Body Slam
- Counter
- Double-Edge
- Endure
- Mud-Slap
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
</div>
</div>
<style>
#pokemon-tabs-bastiodon-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-bastiodon-panel-0 { display: block; }
#pokemon-tabs-bastiodon-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-bastiodon-panel-1 { display: block; }
</style>
</details>
