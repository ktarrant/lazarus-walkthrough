<details class="pokemon-card-container">
<summary>Basculin White (#287)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-basculin-white">
<input type="radio" name="pokemon-tabs-basculin-white-group" id="pokemon-tabs-basculin-white-tab-0" checked>
<label for="pokemon-tabs-basculin-white-tab-0">Basculin White</label>
<input type="radio" name="pokemon-tabs-basculin-white-group" id="pokemon-tabs-basculin-white-tab-1">
<label for="pokemon-tabs-basculin-white-tab-1">Basculegion♂</label>
<input type="radio" name="pokemon-tabs-basculin-white-group" id="pokemon-tabs-basculin-white-tab-2">
<label for="pokemon-tabs-basculin-white-tab-2">Basculegion♀</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-basculin-white-panel-0">
Types: Water • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Rattled
- Adaptability
- Mold Breaker *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=water-pulse">TM03 - Water Pulse</a>
- <a href="move-lookup.html?q=whirlpool">TM07 - Whirlpool</a>
- <a href="move-lookup.html?q=ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=blizzard">TM14 - Blizzard</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=waterfall">HM07 - Waterfall</a>

**Held Item**
Deep Sea Scale
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="basculin-white" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-high">92</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">55</span> |
| Speed | <span class="stat-value stat-high">98</span> |
| Total | <span class="stat-value stat-mid">460</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 1)
- <a href="move-lookup.html?q=water-gun">Water Gun</a> (Lv 1)
- <a href="move-lookup.html?q=uproar">Uproar</a> (Lv 3)
- <a href="move-lookup.html?q=headbutt">Headbutt</a> (Lv 5)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 7)
- <a href="move-lookup.html?q=aqua-jet">Aqua Jet</a> (Lv 9)
- <a href="move-lookup.html?q=chip-away">Chip Away</a> (Lv 11)
- <a href="move-lookup.html?q=take-down">Take Down</a> (Lv 14)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 17)
- <a href="move-lookup.html?q=aqua-tail">Aqua Tail</a> (Lv 20)
- <a href="move-lookup.html?q=soak">Soak</a> (Lv 23)
- <a href="move-lookup.html?q=double-edge">Double-Edge</a> (Lv 26)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 28)
- <a href="move-lookup.html?q=wave-crash">Wave Crash</a> (Lv 32)
- <a href="move-lookup.html?q=flail">Flail</a> (Lv 34)
- <a href="move-lookup.html?q=final-gambit">Final Gambit</a> (Lv 38)
- <a href="move-lookup.html?q=zen-headbutt">Zen Headbutt</a> (Lv 40)
- <a href="move-lookup.html?q=thrash">Thrash</a> (Lv 42)
- <a href="move-lookup.html?q=head-smash">Head Smash</a> (Lv 46)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-basculin-white-panel-1">
Types: Water / Ghost • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Swift Swim
- Adaptability
- Mold Breaker *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=water-pulse">TM03 - Water Pulse</a>
- <a href="move-lookup.html?q=calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=whirlpool">TM07 - Whirlpool</a>
- <a href="move-lookup.html?q=ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=blizzard">TM14 - Blizzard</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=hex">TM23 - Hex</a>
- <a href="move-lookup.html?q=psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=waterfall">HM07 - Waterfall</a>

**Evolution Info**
Lv. after 294 Recoil Dmg, Male
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="basculegion-m" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">120</span> |
| Attack | <span class="stat-value stat-high">112</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-mid">78</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=bitter-malice">Bitter Malice</a> (Lv Evo)
- <a href="move-lookup.html?q=shadow-force">Shadow Force</a> (Lv Evo)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 1)
- <a href="move-lookup.html?q=water-gun">Water Gun</a> (Lv 1)
- <a href="move-lookup.html?q=uproar">Uproar</a> (Lv 3)
- <a href="move-lookup.html?q=headbutt">Headbutt</a> (Lv 5)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 7)
- <a href="move-lookup.html?q=aqua-jet">Aqua Jet</a> (Lv 9)
- <a href="move-lookup.html?q=chip-away">Chip Away</a> (Lv 11)
- <a href="move-lookup.html?q=take-down">Take Down</a> (Lv 14)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 17)
- <a href="move-lookup.html?q=aqua-tail">Aqua Tail</a> (Lv 20)
- <a href="move-lookup.html?q=soak">Soak</a> (Lv 23)
- <a href="move-lookup.html?q=double-edge">Double-Edge</a> (Lv 26)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 28)
- <a href="move-lookup.html?q=wave-crash">Wave Crash</a> (Lv 32)
- <a href="move-lookup.html?q=flail">Flail</a> (Lv 34)
- <a href="move-lookup.html?q=final-gambit">Final Gambit</a> (Lv 38)
- <a href="move-lookup.html?q=phantom-force">Phantom Force</a> (Lv 40)
- <a href="move-lookup.html?q=zen-headbutt">Zen Headbutt</a> (Lv 42)
- <a href="move-lookup.html?q=shadow-ball">Shadow Ball</a> (Lv 45)
- <a href="move-lookup.html?q=thrash">Thrash</a> (Lv 48)
- <a href="move-lookup.html?q=head-smash">Head Smash</a> (Lv 53)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-basculin-white-panel-2">
Types: Water / Ghost • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Swift Swim
- Adaptability
- Mold Breaker *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=water-pulse">TM03 - Water Pulse</a>
- <a href="move-lookup.html?q=calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=whirlpool">TM07 - Whirlpool</a>
- <a href="move-lookup.html?q=ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=blizzard">TM14 - Blizzard</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=hex">TM23 - Hex</a>
- <a href="move-lookup.html?q=psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=waterfall">HM07 - Waterfall</a>

**Evolution Info**
Lv. after 294 Recoil Dmg, Female
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="basculegion-f" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">125</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-high">92</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-mid">78</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=bitter-malice">Bitter Malice</a> (Lv Evo)
- <a href="move-lookup.html?q=shadow-force">Shadow Force</a> (Lv Evo)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 1)
- <a href="move-lookup.html?q=water-gun">Water Gun</a> (Lv 1)
- <a href="move-lookup.html?q=uproar">Uproar</a> (Lv 3)
- <a href="move-lookup.html?q=headbutt">Headbutt</a> (Lv 5)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 7)
- <a href="move-lookup.html?q=aqua-jet">Aqua Jet</a> (Lv 9)
- <a href="move-lookup.html?q=chip-away">Chip Away</a> (Lv 11)
- <a href="move-lookup.html?q=take-down">Take Down</a> (Lv 14)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 17)
- <a href="move-lookup.html?q=aqua-tail">Aqua Tail</a> (Lv 20)
- <a href="move-lookup.html?q=soak">Soak</a> (Lv 23)
- <a href="move-lookup.html?q=double-edge">Double-Edge</a> (Lv 26)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 28)
- <a href="move-lookup.html?q=wave-crash">Wave Crash</a> (Lv 32)
- <a href="move-lookup.html?q=flail">Flail</a> (Lv 34)
- <a href="move-lookup.html?q=final-gambit">Final Gambit</a> (Lv 38)
- <a href="move-lookup.html?q=phantom-force">Phantom Force</a> (Lv 40)
- <a href="move-lookup.html?q=zen-headbutt">Zen Headbutt</a> (Lv 42)
- <a href="move-lookup.html?q=shadow-ball">Shadow Ball</a> (Lv 45)
- <a href="move-lookup.html?q=thrash">Thrash</a> (Lv 48)
- <a href="move-lookup.html?q=head-smash">Head Smash</a> (Lv 53)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
#pokemon-tabs-basculin-white-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-basculin-white-panel-0 { display: block; }
#pokemon-tabs-basculin-white-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-basculin-white-panel-1 { display: block; }
#pokemon-tabs-basculin-white-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-basculin-white-panel-2 { display: block; }
</style>
</details>
