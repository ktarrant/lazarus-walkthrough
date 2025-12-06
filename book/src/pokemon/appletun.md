<details class="pokemon-card-container">
<summary>Appletun (#066)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-appletun">
<input type="radio" name="pokemon-tabs-appletun-group" id="pokemon-tabs-appletun-tab-0">
<label for="pokemon-tabs-appletun-tab-0">Applin</label>
<input type="radio" name="pokemon-tabs-appletun-group" id="pokemon-tabs-appletun-tab-1">
<label for="pokemon-tabs-appletun-tab-1">Flapple</label>
<input type="radio" name="pokemon-tabs-appletun-group" id="pokemon-tabs-appletun-tab-2" checked>
<label for="pokemon-tabs-appletun-tab-2">Appletun</label>
<input type="radio" name="pokemon-tabs-appletun-group" id="pokemon-tabs-appletun-tab-3">
<label for="pokemon-tabs-appletun-tab-3">Dipplin</label>
<input type="radio" name="pokemon-tabs-appletun-group" id="pokemon-tabs-appletun-tab-4">
<label for="pokemon-tabs-appletun-tab-4">Hydrapple</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-appletun-panel-0">
Types: Grass / Dragon • Egg Groups: Grass / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Ripen
- Gluttony
- Bulletproof *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.25×)
- Electric (0.25×)
- Grass (0.25×)
- Ground (0.5×)

*Weak to*
- Ice (4×)
- Poison (2×)
- Flying (2×)
- Bug (2×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>

**Held Item**
Leftovers

**Encounter Locations**
- Kalami City — Grass (Day) (10%)
- Kalami City — Grass (Night) (10%)
- Lastlight Road — Grass (Night) (10%)
- Tower of Dioxippus — Grass (Day) (20%)
- Tower of Dioxippus — Grass (Night) (20%)
- Wanderer's Woods (North) — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="applin" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-low">40</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-low">20</span> |
| Total | <span class="stat-value stat-low">260</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=withdraw">Withdraw</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)

**Egg Moves**
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=recycle">Recycle</a>

**Tutor Moves**
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-appletun-panel-1">
Types: Grass / Dragon • Egg Groups: Grass / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Ripen
- Gluttony
- Poison Touch *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.25×)
- Electric (0.25×)
- Grass (0.25×)
- Ground (0.5×)

*Weak to*
- Ice (4×)
- Poison (2×)
- Flying (2×)
- Bug (2×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=dragon-dance">TM60 - Dragon Dance</a>
- <a href="move-lookup.html?q=fly">HM02 - Fly</a>

**Evolution Info**
Tart Apple

**Encounter Locations**
- Tower of Dioxippus — Grass (Day) (4%)
- Tower of Dioxippus — Grass (Night) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="flapple" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-high">114</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-high">95</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-high">91</span> |
| Total | <span class="stat-value stat-mid">510</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=wing-attack">Wing Attack</a> (Lv Evo)
- <a href="move-lookup.html?q=recycle">Recycle</a> (Lv 1)
- <a href="move-lookup.html?q=withdraw">Withdraw</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)
- <a href="move-lookup.html?q=growth">Growth</a> (Lv 1)
- <a href="move-lookup.html?q=twister">Twister</a> (Lv 5)
- <a href="move-lookup.html?q=acid-spray">Acid Spray</a> (Lv 7)
- <a href="move-lookup.html?q=acrobatics">Acrobatics</a> (Lv 10)
- <a href="move-lookup.html?q=leech-seed">Leech Seed</a> (Lv 12)
- <a href="move-lookup.html?q=protect">Protect</a> (Lv 16)
- <a href="move-lookup.html?q=dragon-breath">Dragon Breath</a> (Lv 20)
- <a href="move-lookup.html?q=dragon-dance">Dragon Dance</a> (Lv 24)
- <a href="move-lookup.html?q=dragon-pulse">Dragon Pulse</a> (Lv 28)
- <a href="move-lookup.html?q=grav-apple">Grav Apple</a> (Lv 30)
- <a href="move-lookup.html?q=iron-defense">Iron Defense</a> (Lv 32)
- <a href="move-lookup.html?q=u-turn">U-turn</a> (Lv 35)
- <a href="move-lookup.html?q=fly">Fly</a> (Lv 38)
- <a href="move-lookup.html?q=scale-shot">Scale Shot</a> (Lv 41)
- <a href="move-lookup.html?q=flying-press">Flying Press</a> (Lv 45)
- <a href="move-lookup.html?q=zing-zap">Zing Zap</a> (Lv 50)

**Egg Moves**
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=recycle">Recycle</a>

**Tutor Moves**
- <a href="move-lookup.html?q=acid-spray">Acid Spray</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-appletun-panel-2">
Types: Grass / Dragon • Egg Groups: Grass / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Ripen
- Gluttony
- Thick Layers *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.25×)
- Electric (0.25×)
- Grass (0.25×)
- Ground (0.5×)

*Weak to*
- Ice (4×)
- Poison (2×)
- Flying (2×)
- Bug (2×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=bulldoze">TM49 - Bulldoze</a>

**Evolution Info**
Sweet Apple

**Encounter Locations**
- Tower of Dioxippus — Grass (Day) (4%)
- Tower of Dioxippus — Grass (Night) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="appletun" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">125</span> |
| Attack | <span class="stat-value stat-mid">85</span> |
| Defense | <span class="stat-value stat-mid">85</span> |
| Sp. Atk | <span class="stat-value stat-high">100</span> |
| Sp. Def | <span class="stat-value stat-mid">85</span> |
| Speed | <span class="stat-value stat-low">30</span> |
| Total | <span class="stat-value stat-mid">510</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=headbutt">Headbutt</a> (Lv Evo)
- <a href="move-lookup.html?q=recycle">Recycle</a> (Lv 1)
- <a href="move-lookup.html?q=withdraw">Withdraw</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)
- <a href="move-lookup.html?q=growth">Growth</a> (Lv 1)
- <a href="move-lookup.html?q=sweet-scent">Sweet Scent</a> (Lv 1)
- <a href="move-lookup.html?q=curse">Curse</a> (Lv 5)
- <a href="move-lookup.html?q=stomp">Stomp</a> (Lv 10)
- <a href="move-lookup.html?q=leech-seed">Leech Seed</a> (Lv 12)
- <a href="move-lookup.html?q=protect">Protect</a> (Lv 16)
- <a href="move-lookup.html?q=bullet-seed">Bullet Seed</a> (Lv 20)
- <a href="move-lookup.html?q=recover">Recover</a> (Lv 24)
- <a href="move-lookup.html?q=apple-acid">Apple Acid</a> (Lv 28)
- <a href="move-lookup.html?q=body-slam">Body Slam</a> (Lv 30)
- <a href="move-lookup.html?q=iron-defense">Iron Defense</a> (Lv 32)
- <a href="move-lookup.html?q=slack-off">Slack Off</a> (Lv 35)
- <a href="move-lookup.html?q=dragon-pulse">Dragon Pulse</a> (Lv 38)
- <a href="move-lookup.html?q=energy-ball">Energy Ball</a> (Lv 41)
- <a href="move-lookup.html?q=body-press">Body Press</a> (Lv 45)
- <a href="move-lookup.html?q=matcha-gotcha">Matcha Gotcha</a> (Lv 50)

**Egg Moves**
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=recycle">Recycle</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-appletun-panel-3">
Types: Grass / Dragon • Egg Groups: Grass / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Supersweet Syrup
- Gluttony
- Sticky Hold *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.25×)
- Electric (0.25×)
- Grass (0.25×)
- Ground (0.5×)

*Weak to*
- Ice (4×)
- Poison (2×)
- Flying (2×)
- Bug (2×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>

**Evolution Info**
Syrupy Apple

**Encounter Locations**
- Tower of Dioxippus — Grass (Day) (2%)
- Tower of Dioxippus — Grass (Night) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="dipplin" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-high">110</span> |
| Sp. Atk | <span class="stat-value stat-high">95</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-mid">495</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=double-hit">Double Hit</a> (Lv Evo)
- <a href="move-lookup.html?q=withdraw">Withdraw</a> (Lv 1)
- <a href="move-lookup.html?q=sweet-scent">Sweet Scent</a> (Lv 1)
- <a href="move-lookup.html?q=recycle">Recycle</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)
- <a href="move-lookup.html?q=dragon-tail">Dragon Tail</a> (Lv 5)
- <a href="move-lookup.html?q=growth">Growth</a> (Lv 10)
- <a href="move-lookup.html?q=dragon-breath">Dragon Breath</a> (Lv 12)
- <a href="move-lookup.html?q=protect">Protect</a> (Lv 16)
- <a href="move-lookup.html?q=bullet-seed">Bullet Seed</a> (Lv 20)
- <a href="move-lookup.html?q=syrup-bomb">Syrup Bomb</a> (Lv 28)
- <a href="move-lookup.html?q=dragon-pulse">Dragon Pulse</a> (Lv 32)
- <a href="move-lookup.html?q=recover">Recover</a> (Lv 36)
- <a href="move-lookup.html?q=dragon-cheer">Dragon Cheer</a> (Lv 38)
- <a href="move-lookup.html?q=energy-ball">Energy Ball</a> (Lv 40)
- <a href="move-lookup.html?q=burning-jealousy">Burning Jealousy</a> (Lv 43)
- <a href="move-lookup.html?q=u-turn">U-turn</a> (Lv 46)
- <a href="move-lookup.html?q=substitute">Substitute</a> (Lv 48)
- <a href="move-lookup.html?q=snap-trap">Snap Trap</a> (Lv 50)
- <a href="move-lookup.html?q=parting-shot">Parting Shot</a> (Lv 54)

**Egg Moves**
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=recycle">Recycle</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-appletun-panel-4">
Types: Grass / Dragon • Egg Groups: Grass / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Supersweet Syrup
- Regenerator
- Sticky Hold *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.25×)
- Electric (0.25×)
- Grass (0.25×)
- Ground (0.5×)

*Weak to*
- Ice (4×)
- Poison (2×)
- Flying (2×)
- Bug (2×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>

**Evolution Info**
Lv. 40
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hydrapple" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">116</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-high">110</span> |
| Sp. Atk | <span class="stat-value stat-high">120</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-low">44</span> |
| Total | <span class="stat-value stat-high">550</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=fickle-beam">Fickle Beam</a> (Lv Evo)
- <a href="move-lookup.html?q=withdraw">Withdraw</a> (Lv 1)
- <a href="move-lookup.html?q=sweet-scent">Sweet Scent</a> (Lv 1)
- <a href="move-lookup.html?q=recycle">Recycle</a> (Lv 1)
- <a href="move-lookup.html?q=astonish">Astonish</a> (Lv 1)
- <a href="move-lookup.html?q=dragon-tail">Dragon Tail</a> (Lv 4)
- <a href="move-lookup.html?q=growth">Growth</a> (Lv 8)
- <a href="move-lookup.html?q=dragon-breath">Dragon Breath</a> (Lv 12)
- <a href="move-lookup.html?q=protect">Protect</a> (Lv 16)
- <a href="move-lookup.html?q=bullet-seed">Bullet Seed</a> (Lv 20)
- <a href="move-lookup.html?q=syrup-bomb">Syrup Bomb</a> (Lv 28)
- <a href="move-lookup.html?q=dragon-pulse">Dragon Pulse</a> (Lv 32)
- <a href="move-lookup.html?q=recover">Recover</a> (Lv 36)
- <a href="move-lookup.html?q=dragon-cheer">Dragon Cheer</a> (Lv 38)
- <a href="move-lookup.html?q=energy-ball">Energy Ball</a> (Lv 40)
- <a href="move-lookup.html?q=burning-jealousy">Burning Jealousy</a> (Lv 43)
- <a href="move-lookup.html?q=u-turn">U-turn</a> (Lv 46)
- <a href="move-lookup.html?q=substitute">Substitute</a> (Lv 48)
- <a href="move-lookup.html?q=snap-trap">Snap Trap</a> (Lv 50)
- <a href="move-lookup.html?q=parting-shot">Parting Shot</a> (Lv 54)

**Egg Moves**
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=recycle">Recycle</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
#pokemon-tabs-appletun-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-appletun-panel-0 { display: block; }
#pokemon-tabs-appletun-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-appletun-panel-1 { display: block; }
#pokemon-tabs-appletun-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-appletun-panel-2 { display: block; }
#pokemon-tabs-appletun-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-appletun-panel-3 { display: block; }
#pokemon-tabs-appletun-tab-4:checked ~ .pokemon-tab-panels #pokemon-tabs-appletun-panel-4 { display: block; }
</style>
</details>
