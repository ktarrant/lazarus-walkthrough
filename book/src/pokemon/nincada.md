<details class="pokemon-card-container">
<summary>Nincada (#045)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-nincada">
<input type="radio" name="pokemon-tabs-nincada-group" id="pokemon-tabs-nincada-tab-0" checked>
<label for="pokemon-tabs-nincada-tab-0">Nincada</label>
<input type="radio" name="pokemon-tabs-nincada-group" id="pokemon-tabs-nincada-tab-1">
<label for="pokemon-tabs-nincada-tab-1">Ninjask</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-nincada-panel-0">
Types: Bug / Ground • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Compound Eyes
- Shed Skin
- Run Away *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Fighting (0.5×)
- Poison (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Water (2×)
- Ice (2×)
- Flying (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm30-shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm37-sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=tm40-aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=hm01-cut">HM01 - Cut</a>
- <a href="move-lookup.html?q=hm05-flash">HM05 - Flash</a>

**Held Item**
Soft Sand

**Encounter Locations**
- Bronze Fields (North) — Grass (Day) (20%)
- Bronze Fields (South) — Grass (Day) (10%)
- Bronze Fields (South) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="nincada" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">31</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-low">30</span> |
| Sp. Def | <span class="stat-value stat-low">30</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-low">266</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=harden">Harden</a> (Lv 1)
- <a href="move-lookup.html?q=absorb">Absorb</a> (Lv 5)
- <a href="move-lookup.html?q=sand-attack">Sand Attack</a> (Lv 9)
- <a href="move-lookup.html?q=fury-swipes">Fury Swipes</a> (Lv 13)
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a> (Lv 17)
- <a href="move-lookup.html?q=metal-claw">Metal Claw</a> (Lv 21)
- <a href="move-lookup.html?q=mind-reader">Mind Reader</a> (Lv 25)
- <a href="move-lookup.html?q=bide">Bide</a> (Lv 29)
- <a href="move-lookup.html?q=false-swipe">False Swipe</a> (Lv 33)
- <a href="move-lookup.html?q=dig">Dig</a> (Lv 37)

**Egg Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=feint-attack">Feint Attack</a>
- <a href="move-lookup.html?q=gust">Gust</a>
- <a href="move-lookup.html?q=silver-wind">Silver Wind</a>
- <a href="move-lookup.html?q=bug-buzz">Bug Buzz</a>
- <a href="move-lookup.html?q=night-slash">Night Slash</a>
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a>
- <a href="move-lookup.html?q=final-gambit">Final Gambit</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-nincada-panel-1">
Types: Bug / Dark • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Speed Boost
- Sharpness
- Infiltrator *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Ground (0.5×)
- Psychic (0×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Fire (2×)
- Flying (2×)
- Bug (2×)
- Rock (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm30-shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm37-sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=tm40-aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm57-roost">TM57 - Roost</a>
- <a href="move-lookup.html?q=hm01-cut">HM01 - Cut</a>
- <a href="move-lookup.html?q=hm05-flash">HM05 - Flash</a>

**Evolution Info**
Lv. 20

**Encounter Locations**
- Sea of Asteri (East) — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ninjask" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">61</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-high">160</span> |
| Total | <span class="stat-value stat-mid">466</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a> (Lv Evo)
- <a href="move-lookup.html?q=screech">Screech</a> (Lv Evo)
- <a href="move-lookup.html?q=double-team">Double Team</a> (Lv Evo)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 1)
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=harden">Harden</a> (Lv 1)
- <a href="move-lookup.html?q=absorb">Absorb</a> (Lv 5)
- <a href="move-lookup.html?q=sand-attack">Sand Attack</a> (Lv 9)
- <a href="move-lookup.html?q=fury-swipes">Fury Swipes</a> (Lv 13)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 17)
- <a href="move-lookup.html?q=slash">Slash</a> (Lv 23)
- <a href="move-lookup.html?q=night-slash">Night Slash</a> (Lv 25)
- <a href="move-lookup.html?q=fell-stinger">Fell Stinger</a> (Lv 27)
- <a href="move-lookup.html?q=mind-reader">Mind Reader</a> (Lv 29)
- <a href="move-lookup.html?q=x-scissor">X-Scissor</a> (Lv 33)
- <a href="move-lookup.html?q=quiver-dance">Quiver Dance</a> (Lv 35)
- <a href="move-lookup.html?q=baton-pass">Baton Pass</a> (Lv 35)
- <a href="move-lookup.html?q=knock-off">Knock Off</a> (Lv 37)
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a> (Lv 41)
- <a href="move-lookup.html?q=foul-play">Foul Play</a> (Lv 43)
- <a href="move-lookup.html?q=attack-order">Attack Order</a> (Lv 47)
- <a href="move-lookup.html?q=defend-order">Defend Order</a> (Lv 47)
- <a href="move-lookup.html?q=heal-order">Heal Order</a> (Lv 47)

**Egg Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=feint-attack">Feint Attack</a>
- <a href="move-lookup.html?q=gust">Gust</a>
- <a href="move-lookup.html?q=silver-wind">Silver Wind</a>
- <a href="move-lookup.html?q=bug-buzz">Bug Buzz</a>
- <a href="move-lookup.html?q=night-slash">Night Slash</a>
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a>
- <a href="move-lookup.html?q=final-gambit">Final Gambit</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
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
#pokemon-tabs-nincada-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-nincada-panel-0 { display: block; }
#pokemon-tabs-nincada-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-nincada-panel-1 { display: block; }
</style>
</details>
