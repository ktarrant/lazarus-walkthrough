<details class="pokemon-card-container">
<summary>Silvally (#413)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-silvally">
<input type="radio" name="pokemon-tabs-silvally-group" id="pokemon-tabs-silvally-tab-0">
<label for="pokemon-tabs-silvally-tab-0">Type: Null</label>
<input type="radio" name="pokemon-tabs-silvally-group" id="pokemon-tabs-silvally-tab-1" checked>
<label for="pokemon-tabs-silvally-tab-1">Silvally</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-silvally-panel-0">
Types: Normal • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Battle Armor

**Type Matchups**

*Resists / Immune to*
- Ghost (0×)

*Weak to*
- Fighting (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm02-dragon-claw">TM02 - Dragon Claw</a>
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm37-sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=tm40-aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="type-null" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">85</span> |
| Attack | <span class="stat-value stat-mid">85</span> |
| Defense | <span class="stat-value stat-mid">85</span> |
| Sp. Atk | <span class="stat-value stat-mid">85</span> |
| Sp. Def | <span class="stat-value stat-mid">85</span> |
| Speed | <span class="stat-value stat-mid">55</span> |
| Total | <span class="stat-value stat-mid">480</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=rage">Rage</a> (Lv 5)
- <a href="move-lookup.html?q=pursuit">Pursuit</a> (Lv 10)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 13)
- <a href="move-lookup.html?q=imprison">Imprison</a> (Lv 15)
- <a href="move-lookup.html?q=aerial-ace">Aerial Ace</a> (Lv 16)
- <a href="move-lookup.html?q=crush-claw">Crush Claw</a> (Lv 20)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 23)
- <a href="move-lookup.html?q=x-scissor">X-Scissor</a> (Lv 26)
- <a href="move-lookup.html?q=take-down">Take Down</a> (Lv 30)
- <a href="move-lookup.html?q=metal-sound">Metal Sound</a> (Lv 33)
- <a href="move-lookup.html?q=iron-head">Iron Head</a> (Lv 36)
- <a href="move-lookup.html?q=double-hit">Double Hit</a> (Lv 40)
- <a href="move-lookup.html?q=air-slash">Air Slash</a> (Lv 43)
- <a href="move-lookup.html?q=punishment">Punishment</a> (Lv 46)
- <a href="move-lookup.html?q=razor-wind">Razor Wind</a> (Lv 50)
- <a href="move-lookup.html?q=tri-attack">Tri Attack</a> (Lv 53)
- <a href="move-lookup.html?q=double-edge">Double-Edge</a> (Lv 56)
- <a href="move-lookup.html?q=heal-block">Heal Block</a> (Lv 60)
- <a href="move-lookup.html?q=parting-shot">Parting Shot</a> (Lv 65)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swift">Swift</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-silvally-panel-1">
Types: Normal • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- RKS System

**Type Matchups**

*Resists / Immune to*
- Ghost (0×)

*Weak to*
- Fighting (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm02-dragon-claw">TM02 - Dragon Claw</a>
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm13-ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm30-shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm35-flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=tm37-sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=tm40-aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm43-poison-fang">TM43 - Poison Fang</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm47-steel-wing">TM47 - Steel Wing</a>
- <a href="move-lookup.html?q=tm55-snarl">TM55 - Snarl</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=hm03-surf">HM03 - Surf</a>

**Evolution Info**
Lv. w/ High Friendship?
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="silvally" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">95</span> |
| Defense | <span class="stat-value stat-high">95</span> |
| Sp. Atk | <span class="stat-value stat-high">95</span> |
| Sp. Def | <span class="stat-value stat-high">95</span> |
| Speed | <span class="stat-value stat-high">95</span> |
| Total | <span class="stat-value stat-high">570</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=multi-attack">Multi-Attack</a> (Lv Evo)
- <a href="move-lookup.html?q=heal-block">Heal Block</a> (Lv 1)
- <a href="move-lookup.html?q=imprison">Imprison</a> (Lv 1)
- <a href="move-lookup.html?q=iron-head">Iron Head</a> (Lv 1)
- <a href="move-lookup.html?q=poison-fang">Poison Fang</a> (Lv 1)
- <a href="move-lookup.html?q=fire-fang">Fire Fang</a> (Lv 1)
- <a href="move-lookup.html?q=ice-fang">Ice Fang</a> (Lv 1)
- <a href="move-lookup.html?q=thunder-fang">Thunder Fang</a> (Lv 1)
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=rage">Rage</a> (Lv 5)
- <a href="move-lookup.html?q=pursuit">Pursuit</a> (Lv 10)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 13)
- <a href="move-lookup.html?q=aerial-ace">Aerial Ace</a> (Lv 16)
- <a href="move-lookup.html?q=crush-claw">Crush Claw</a> (Lv 20)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 23)
- <a href="move-lookup.html?q=x-scissor">X-Scissor</a> (Lv 26)
- <a href="move-lookup.html?q=take-down">Take Down</a> (Lv 30)
- <a href="move-lookup.html?q=metal-sound">Metal Sound</a> (Lv 33)
- <a href="move-lookup.html?q=iron-head">Iron Head</a> (Lv 36)
- <a href="move-lookup.html?q=double-hit">Double Hit</a> (Lv 40)
- <a href="move-lookup.html?q=air-slash">Air Slash</a> (Lv 43)
- <a href="move-lookup.html?q=punishment">Punishment</a> (Lv 46)
- <a href="move-lookup.html?q=razor-wind">Razor Wind</a> (Lv 50)
- <a href="move-lookup.html?q=tri-attack">Tri Attack</a> (Lv 53)
- <a href="move-lookup.html?q=double-edge">Double-Edge</a> (Lv 56)
- <a href="move-lookup.html?q=heal-block">Heal Block</a> (Lv 60)
- <a href="move-lookup.html?q=parting-shot">Parting Shot</a> (Lv 65)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=explosion">Explosion</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swift">Swift</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
#pokemon-tabs-silvally-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-silvally-panel-0 { display: block; }
#pokemon-tabs-silvally-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-silvally-panel-1 { display: block; }
</style>
</details>
