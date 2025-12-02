<details class="pokemon-card-container">
<summary>Grubbin (#048)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-grubbin">
<input type="radio" name="pokemon-tabs-grubbin-group" id="pokemon-tabs-grubbin-tab-0" checked>
<label for="pokemon-tabs-grubbin-tab-0">Grubbin</label>
<input type="radio" name="pokemon-tabs-grubbin-group" id="pokemon-tabs-grubbin-tab-1">
<label for="pokemon-tabs-grubbin-tab-1">Charjabug</label>
<input type="radio" name="pokemon-tabs-grubbin-group" id="pokemon-tabs-grubbin-tab-2">
<label for="pokemon-tabs-grubbin-tab-2">Vikavolt</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-grubbin-panel-0">
Types: Bug • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Swarm
- Swarm *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Flying (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm16-light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm20-poison-jab">TM20 - Poison Jab</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm34-shock-wave">TM34 - Shock Wave</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>

**Encounter Locations**
- Bronze Fields (North) — Grass (Day) (10%)
- Bronze Fields (North) — Grass (Night) (10%)
- Bronze Fields (South) — Grass (Day) (4%)
- Bronze Fields (South) — Grass (Night) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="grubbin" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">47</span> |
| Attack | <span class="stat-value stat-mid">62</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-low">45</span> |
| Speed | <span class="stat-value stat-low">46</span> |
| Total | <span class="stat-value stat-low">300</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=vise-grip">Vise Grip</a> (Lv 1)
- <a href="move-lookup.html?q=string-shot">String Shot</a> (Lv 4)
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a> (Lv 7)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 10)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 13)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 16)
- <a href="move-lookup.html?q=acrobatics">Acrobatics</a> (Lv 19)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 22)
- <a href="move-lookup.html?q=x-scissor">X-Scissor</a> (Lv 25)
- <a href="move-lookup.html?q=dig">Dig</a> (Lv 28)

**Egg Moves**
- <a href="move-lookup.html?q=harden">Harden</a>
- <a href="move-lookup.html?q=electroweb">Electroweb</a>
- <a href="move-lookup.html?q=mud-shot">Mud Shot</a>
- <a href="move-lookup.html?q=endure">Endure</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-grubbin-panel-1">
Types: Bug / Electric • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Battery
- Battery *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Grass (0.5×)
- Fighting (0.5×)
- Steel (0.5×)

*Weak to*
- Fire (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm16-light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm20-poison-jab">TM20 - Poison Jab</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm25-thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm34-shock-wave">TM34 - Shock Wave</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>

**Held Item**
Cell Battery

**Evolution Info**
Lv. 20

**Encounter Locations**
- Kaptara Island (East) — Grass (Day) (20%)
- Kaptara Island (East) — Grass (Night) (20%)
- Pollen Road — Grass (Night) (10%)
- Sea of Asteri (East) — Grass (Day) (10%)
- Sea of Asteri (East) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="charjabug" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">57</span> |
| Attack | <span class="stat-value stat-mid">72</span> |
| Defense | <span class="stat-value stat-high">95</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-low">31</span> |
| Total | <span class="stat-value stat-mid">400</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=charge">Charge</a> (Lv Evo)
- <a href="move-lookup.html?q=vise-grip">Vise Grip</a> (Lv 1)
- <a href="move-lookup.html?q=string-shot">String Shot</a> (Lv 4)
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a> (Lv 7)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 10)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 13)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 16)
- <a href="move-lookup.html?q=acrobatics">Acrobatics</a> (Lv 19)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 25)
- <a href="move-lookup.html?q=x-scissor">X-Scissor</a> (Lv 31)
- <a href="move-lookup.html?q=dig">Dig</a> (Lv 37)
- <a href="move-lookup.html?q=discharge">Discharge</a> (Lv 43)
- <a href="move-lookup.html?q=iron-defense">Iron Defense</a> (Lv 49)

**Egg Moves**
- <a href="move-lookup.html?q=harden">Harden</a>
- <a href="move-lookup.html?q=electroweb">Electroweb</a>
- <a href="move-lookup.html?q=mud-shot">Mud Shot</a>
- <a href="move-lookup.html?q=endure">Endure</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-grubbin-panel-2">
Types: Bug / Electric • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Levitate
- Speed Boost *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Grass (0.5×)
- Fighting (0.5×)
- Steel (0.5×)

*Weak to*
- Fire (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm16-light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm20-poison-jab">TM20 - Poison Jab</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm24-thunderbolt">TM24 - Thunderbolt</a>
- <a href="move-lookup.html?q=tm25-thunder">TM25 - Thunder</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm34-shock-wave">TM34 - Shock Wave</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm49-bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=tm57-roost">TM57 - Roost</a>
- <a href="move-lookup.html?q=tm58-thunder-wave">TM58 - Thunder Wave</a>
- <a href="move-lookup.html?q=hm02-fly">HM02 - Fly</a>

**Evolution Info**
Thunder Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="vikavolt" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">77</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-high">135</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-low">43</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=thunderbolt">Thunderbolt</a> (Lv Evo)
- <a href="move-lookup.html?q=air-slash">Air Slash</a> (Lv 1)
- <a href="move-lookup.html?q=charge">Charge</a> (Lv 1)
- <a href="move-lookup.html?q=vise-grip">Vise Grip</a> (Lv 1)
- <a href="move-lookup.html?q=string-shot">String Shot</a> (Lv 4)
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a> (Lv 7)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 10)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 13)
- <a href="move-lookup.html?q=spark">Spark</a> (Lv 16)
- <a href="move-lookup.html?q=acrobatics">Acrobatics</a> (Lv 19)
- <a href="move-lookup.html?q=guillotine">Guillotine</a> (Lv 25)
- <a href="move-lookup.html?q=bug-buzz">Bug Buzz</a> (Lv 31)
- <a href="move-lookup.html?q=dig">Dig</a> (Lv 37)
- <a href="move-lookup.html?q=zap-cannon">Zap Cannon</a> (Lv 41)
- <a href="move-lookup.html?q=u-turn">U-Turn</a> (Lv 45)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 49)
- <a href="move-lookup.html?q=supercell-slam">Supercell Slam</a> (Lv 52)

**Egg Moves**
- <a href="move-lookup.html?q=harden">Harden</a>
- <a href="move-lookup.html?q=electroweb">Electroweb</a>
- <a href="move-lookup.html?q=mud-shot">Mud Shot</a>
- <a href="move-lookup.html?q=endure">Endure</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
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
#pokemon-tabs-grubbin-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-grubbin-panel-0 { display: block; }
#pokemon-tabs-grubbin-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-grubbin-panel-1 { display: block; }
#pokemon-tabs-grubbin-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-grubbin-panel-2 { display: block; }
</style>
</details>
