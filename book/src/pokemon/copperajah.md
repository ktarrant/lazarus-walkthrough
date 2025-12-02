<details class="pokemon-card-container">
<summary>Copperajah (#197)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-copperajah">
<input type="radio" name="pokemon-tabs-copperajah-group" id="pokemon-tabs-copperajah-tab-0">
<label for="pokemon-tabs-copperajah-tab-0">Cufant</label>
<input type="radio" name="pokemon-tabs-copperajah-group" id="pokemon-tabs-copperajah-tab-1" checked>
<label for="pokemon-tabs-copperajah-tab-1">Copperajah</label>
<input type="radio" name="pokemon-tabs-copperajah-group" id="pokemon-tabs-copperajah-tab-2">
<label for="pokemon-tabs-copperajah-tab-2">Mega Copperajah</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-copperajah-panel-0">
Types: Steel / Normal • Egg Groups: Field / Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sheer Force
- Long Reach
- Heavy Metal *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.5×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Ghost (0×)
- Dragon (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Fighting (4×)
- Ground (2×)

**TM/HM Moves**
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM04 - Strength
- HM06 - Rock Smash

**Held Item**
Lagging Tail

**Encounter Locations**
- Riverwalk Trail (South) — Grass (Day) (5%)
- Riverwalk Trail (South) — Grass (Night) (5%)
- Sea of Asteri (East) — Grass (Day) (10%)
- Sea of Asteri (West) — Grass (Night) (15%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="cufant" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">72</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-low">49</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">49</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-mid">330</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Growl (Lv 1)
- Rollout (Lv 5)
- Rock Smash (Lv 10)
- Water Gun (Lv 13)
- Bulldoze (Lv 16)
- Stomp (Lv 20)
- Iron Defense (Lv 23)
- Body Slam (Lv 27)
- Dig (Lv 30)
- Strength (Lv 34)
- Iron Head (Lv 38)
- Play Rough (Lv 42)
- Liquidation (Lv 45)
- High Horsepower (Lv 49)
- Megahorn (Lv 53)
- Superpower (Lv 55)

**Egg Moves**
- Double-Edge
- Belch
- Curse
- Slam
- Fissure
- Swagger
- Whirlwind
- Defense Curl

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Mega Kick
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
<div class="pokemon-tab-panel" id="pokemon-tabs-copperajah-panel-1">
Types: Steel / Normal • Egg Groups: Field / Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sheer Force
- Long Reach
- Heavy Metal *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.5×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Ghost (0×)
- Dragon (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Fighting (4×)
- Ground (2×)

**TM/HM Moves**
- TM12 - Taunt
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM55 - Snarl
- HM04 - Strength
- HM06 - Rock Smash

**Held Item**
Lagging Tail

**Evolution Info**
Lv. 34
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="copperajah" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">122</span> |
| Attack | <span class="stat-value stat-high">130</span> |
| Defense | <span class="stat-value stat-mid">74</span> |
| Sp. Atk | <span class="stat-value stat-mid">75</span> |
| Sp. Def | <span class="stat-value stat-mid">69</span> |
| Speed | <span class="stat-value stat-low">30</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- Heavy Slam (Lv Evo)
- Body Press (Lv 1)
- Tackle (Lv 1)
- Growl (Lv 1)
- Rollout (Lv 5)
- Rock Smash (Lv 10)
- Water Gun (Lv 13)
- Bulldoze (Lv 16)
- Stomp (Lv 20)
- Iron Defense (Lv 23)
- Body Slam (Lv 27)
- Dig (Lv 30)
- Strength (Lv 34)
- Iron Head (Lv 38)
- Play Rough (Lv 42)
- Liquidation (Lv 45)
- High Horsepower (Lv 49)
- Megahorn (Lv 53)
- Superpower (Lv 55)

**Egg Moves**
- Double-Edge
- Belch
- Curse
- Slam
- Fissure
- Swagger
- Whirlwind
- Defense Curl

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Mega Kick
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
<div class="pokemon-tab-panel" id="pokemon-tabs-copperajah-panel-2">
Types: Steel / Normal • Egg Groups: Field / Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Super Luck

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.5×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Ghost (0×)
- Dragon (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Fighting (4×)
- Ground (2×)

**TM/HM Moves**
- TM12 - Taunt
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM55 - Snarl
- HM04 - Strength
- HM06 - Rock Smash

**Held Item**
Lagging Tail

**Evolution Info**
Copperajite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-copperajah" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">122</span> |
| Attack | <span class="stat-value stat-high">150</span> |
| Defense | <span class="stat-value stat-high">109</span> |
| Sp. Atk | <span class="stat-value stat-mid">85</span> |
| Sp. Def | <span class="stat-value stat-mid">89</span> |
| Speed | <span class="stat-value stat-low">45</span> |
| Total | <span class="stat-value stat-high">600</span> |

**Level-Up Moves**
- Heavy Slam (Lv Evo)
- Body Press (Lv 1)
- Tackle (Lv 1)
- Growl (Lv 1)
- Rollout (Lv 5)
- Rock Smash (Lv 10)
- Water Gun (Lv 13)
- Bulldoze (Lv 16)
- Stomp (Lv 20)
- Iron Defense (Lv 23)
- Body Slam (Lv 27)
- Dig (Lv 30)
- Strength (Lv 34)
- Iron Head (Lv 38)
- Play Rough (Lv 42)
- Liquidation (Lv 45)
- High Horsepower (Lv 49)
- Megahorn (Lv 53)
- Superpower (Lv 55)

**Egg Moves**
- Double-Edge
- Belch
- Curse
- Slam
- Fissure
- Swagger
- Whirlwind
- Defense Curl

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Mega Kick
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
#pokemon-tabs-copperajah-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-copperajah-panel-0 { display: block; }
#pokemon-tabs-copperajah-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-copperajah-panel-1 { display: block; }
#pokemon-tabs-copperajah-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-copperajah-panel-2 { display: block; }
</style>
</details>
