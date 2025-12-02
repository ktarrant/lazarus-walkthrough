<details class="pokemon-card-container">
<summary>Flabébé (#312)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-flabebe">
<input type="radio" name="pokemon-tabs-flabebe-group" id="pokemon-tabs-flabebe-tab-0" checked>
<label for="pokemon-tabs-flabebe-tab-0">Flabébé</label>
<input type="radio" name="pokemon-tabs-flabebe-group" id="pokemon-tabs-flabebe-tab-1">
<label for="pokemon-tabs-flabebe-tab-1">Floette</label>
<input type="radio" name="pokemon-tabs-flabebe-group" id="pokemon-tabs-flabebe-tab-2">
<label for="pokemon-tabs-flabebe-tab-2">Florges</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-flabebe-panel-0">
Types: Fairy • Egg Groups: Fairy

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flower Veil
- Symbiosis *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Bug (0.5×)
- Dragon (0×)
- Dark (0.5×)

*Weak to*
- Poison (2×)
- Steel (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm01-wish">TM01 - Wish</a>
- <a href="move-lookup.html?q=tm04-calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm15-draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=tm16-light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm29-psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm54-dazzling-gleam">TM54 - Dazzling Gleam</a>
- <a href="move-lookup.html?q=hm05-flash">HM05 - Flash</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="flabebe" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">44</span> |
| Attack | <span class="stat-value stat-low">38</span> |
| Defense | <span class="stat-value stat-low">39</span> |
| Sp. Atk | <span class="stat-value stat-mid">61</span> |
| Sp. Def | <span class="stat-value stat-mid">79</span> |
| Speed | <span class="stat-value stat-low">42</span> |
| Total | <span class="stat-value stat-low">303</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=vine-whip">Vine Whip</a> (Lv 1)
- <a href="move-lookup.html?q=fairy-wind">Fairy Wind</a> (Lv 6)
- <a href="move-lookup.html?q=lucky-chant">Lucky Chant</a> (Lv 10)
- <a href="move-lookup.html?q=razor-leaf">Razor Leaf</a> (Lv 15)
- <a href="move-lookup.html?q=wish">Wish</a> (Lv 20)
- <a href="move-lookup.html?q=magical-leaf">Magical Leaf</a> (Lv 22)
- <a href="move-lookup.html?q=grassy-terrain">Grassy Terrain</a> (Lv 24)
- <a href="move-lookup.html?q=petal-blizzard">Petal Blizzard</a> (Lv 28)
- <a href="move-lookup.html?q=aromatherapy">Aromatherapy</a> (Lv 33)
- <a href="move-lookup.html?q=misty-terrain">Misty Terrain</a> (Lv 37)
- <a href="move-lookup.html?q=moonblast">Moonblast</a> (Lv 41)
- <a href="move-lookup.html?q=petal-dance">Petal Dance</a> (Lv 45)
- <a href="move-lookup.html?q=solar-beam">Solar Beam</a> (Lv 48)

**Egg Moves**
- <a href="move-lookup.html?q=copycat">Copycat</a>
- <a href="move-lookup.html?q=captivate">Captivate</a>
- <a href="move-lookup.html?q=camouflage">Camouflage</a>
- <a href="move-lookup.html?q=tearful-look">Tearful Look</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-flabebe-panel-1">
Types: Fairy • Egg Groups: Fairy

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flower Veil
- Symbiosis *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Bug (0.5×)
- Dragon (0×)
- Dark (0.5×)

*Weak to*
- Poison (2×)
- Steel (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm01-wish">TM01 - Wish</a>
- <a href="move-lookup.html?q=tm04-calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm15-draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=tm16-light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm29-psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm48-skill-swap">TM48 - Skill Swap</a>
- <a href="move-lookup.html?q=tm54-dazzling-gleam">TM54 - Dazzling Gleam</a>
- <a href="move-lookup.html?q=hm05-flash">HM05 - Flash</a>

**Evolution Info**
Lv. 19

**Encounter Locations**
- Port Pello — Grass (Night) (20%)
- Péntepetal City — Grass (Day) (20%)
- Péntepetal City — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="floette" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">54</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-low">47</span> |
| Sp. Atk | <span class="stat-value stat-mid">75</span> |
| Sp. Def | <span class="stat-value stat-high">98</span> |
| Speed | <span class="stat-value stat-mid">52</span> |
| Total | <span class="stat-value stat-mid">371</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=draining-kiss">Draining Kiss</a> (Lv Evo)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=vine-whip">Vine Whip</a> (Lv 1)
- <a href="move-lookup.html?q=fairy-wind">Fairy Wind</a> (Lv 6)
- <a href="move-lookup.html?q=lucky-chant">Lucky Chant</a> (Lv 10)
- <a href="move-lookup.html?q=razor-leaf">Razor Leaf</a> (Lv 15)
- <a href="move-lookup.html?q=wish">Wish</a> (Lv 20)
- <a href="move-lookup.html?q=magical-leaf">Magical Leaf</a> (Lv 23)
- <a href="move-lookup.html?q=grassy-terrain">Grassy Terrain</a> (Lv 25)
- <a href="move-lookup.html?q=synthesis">Synthesis</a> (Lv 28)
- <a href="move-lookup.html?q=petal-blizzard">Petal Blizzard</a> (Lv 33)
- <a href="move-lookup.html?q=aromatherapy">Aromatherapy</a> (Lv 35)
- <a href="move-lookup.html?q=pollen-puff">Pollen Puff</a> (Lv 38)
- <a href="move-lookup.html?q=misty-terrain">Misty Terrain</a> (Lv 43)
- <a href="move-lookup.html?q=moonblast">Moonblast</a> (Lv 46)
- <a href="move-lookup.html?q=petal-dance">Petal Dance</a> (Lv 51)
- <a href="move-lookup.html?q=solar-beam">Solar Beam</a> (Lv 58)

**Egg Moves**
- <a href="move-lookup.html?q=copycat">Copycat</a>
- <a href="move-lookup.html?q=captivate">Captivate</a>
- <a href="move-lookup.html?q=camouflage">Camouflage</a>
- <a href="move-lookup.html?q=tearful-look">Tearful Look</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=metronome">Metronome</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-flabebe-panel-2">
Types: Fairy • Egg Groups: Fairy

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flower Veil
- Symbiosis *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Bug (0.5×)
- Dragon (0×)
- Dark (0.5×)

*Weak to*
- Poison (2×)
- Steel (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm01-wish">TM01 - Wish</a>
- <a href="move-lookup.html?q=tm04-calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm15-draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=tm16-light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm29-psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm48-skill-swap">TM48 - Skill Swap</a>
- <a href="move-lookup.html?q=tm54-dazzling-gleam">TM54 - Dazzling Gleam</a>
- <a href="move-lookup.html?q=hm05-flash">HM05 - Flash</a>

**Evolution Info**
Shiny Stone

**Encounter Locations**
- Port Pello — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="florges" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">78</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-mid">68</span> |
| Sp. Atk | <span class="stat-value stat-high">112</span> |
| Sp. Def | <span class="stat-value stat-high">154</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-high">552</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=giga-drain">Giga Drain</a> (Lv Evo)
- <a href="move-lookup.html?q=disarming-voice">Disarming Voice</a> (Lv 1)
- <a href="move-lookup.html?q=draining-kiss">Draining Kiss</a> (Lv 1)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=vine-whip">Vine Whip</a> (Lv 1)
- <a href="move-lookup.html?q=fairy-wind">Fairy Wind</a> (Lv 6)
- <a href="move-lookup.html?q=lucky-chant">Lucky Chant</a> (Lv 10)
- <a href="move-lookup.html?q=razor-leaf">Razor Leaf</a> (Lv 15)
- <a href="move-lookup.html?q=wish">Wish</a> (Lv 20)
- <a href="move-lookup.html?q=magical-leaf">Magical Leaf</a> (Lv 23)
- <a href="move-lookup.html?q=grassy-terrain">Grassy Terrain</a> (Lv 25)
- <a href="move-lookup.html?q=synthesis">Synthesis</a> (Lv 28)
- <a href="move-lookup.html?q=petal-blizzard">Petal Blizzard</a> (Lv 33)
- <a href="move-lookup.html?q=aromatherapy">Aromatherapy</a> (Lv 35)
- <a href="move-lookup.html?q=pollen-puff">Pollen Puff</a> (Lv 38)
- <a href="move-lookup.html?q=misty-terrain">Misty Terrain</a> (Lv 43)
- <a href="move-lookup.html?q=moonblast">Moonblast</a> (Lv 46)
- <a href="move-lookup.html?q=petal-dance">Petal Dance</a> (Lv 51)
- <a href="move-lookup.html?q=solar-beam">Solar Beam</a> (Lv 58)

**Egg Moves**
- <a href="move-lookup.html?q=copycat">Copycat</a>
- <a href="move-lookup.html?q=captivate">Captivate</a>
- <a href="move-lookup.html?q=camouflage">Camouflage</a>
- <a href="move-lookup.html?q=tearful-look">Tearful Look</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=metronome">Metronome</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
#pokemon-tabs-flabebe-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-flabebe-panel-0 { display: block; }
#pokemon-tabs-flabebe-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-flabebe-panel-1 { display: block; }
#pokemon-tabs-flabebe-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-flabebe-panel-2 { display: block; }
</style>
</details>
