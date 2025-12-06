<details class="pokemon-card-container">
<summary>Tsareena (#133)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-tsareena">
<input type="radio" name="pokemon-tabs-tsareena-group" id="pokemon-tabs-tsareena-tab-0">
<label for="pokemon-tabs-tsareena-tab-0">Bounsweet</label>
<input type="radio" name="pokemon-tabs-tsareena-group" id="pokemon-tabs-tsareena-tab-1">
<label for="pokemon-tabs-tsareena-tab-1">Steenee</label>
<input type="radio" name="pokemon-tabs-tsareena-group" id="pokemon-tabs-tsareena-tab-2" checked>
<label for="pokemon-tabs-tsareena-tab-2">Tsareena</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-tsareena-panel-0">
Types: Grass • Egg Groups: Grass

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Leaf Guard
- Oblivious
- Sweet Veil *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=dazzling-gleam">TM54 - Dazzling Gleam</a>

**Held Item**
Grassy Seed

**Encounter Locations**
- Jusmail Town — Grass (Day) (10%)
- Riverwalk Trail (South) — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="bounsweet" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">42</span> |
| Attack | <span class="stat-value stat-low">30</span> |
| Defense | <span class="stat-value stat-low">38</span> |
| Sp. Atk | <span class="stat-value stat-low">30</span> |
| Sp. Def | <span class="stat-value stat-low">38</span> |
| Speed | <span class="stat-value stat-low">32</span> |
| Total | <span class="stat-value stat-low">210</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=splash">Splash</a> (Lv 1)
- <a href="move-lookup.html?q=play-nice">Play Nice</a> (Lv 5)
- <a href="move-lookup.html?q=rapid-spin">Rapid Spin</a> (Lv 9)
- <a href="move-lookup.html?q=razor-leaf">Razor Leaf</a> (Lv 13)
- <a href="move-lookup.html?q=synthesis">Synthesis</a> (Lv 15)
- <a href="move-lookup.html?q=sweet-scent">Sweet Scent</a> (Lv 17)
- <a href="move-lookup.html?q=magical-leaf">Magical Leaf</a> (Lv 21)
- <a href="move-lookup.html?q=teeter-dance">Teeter Dance</a> (Lv 25)
- <a href="move-lookup.html?q=stomp">Stomp</a> (Lv 29)
- <a href="move-lookup.html?q=aromatic-mist">Aromatic Mist</a> (Lv 33)

**Egg Moves**
- <a href="move-lookup.html?q=grass-whistle">Grass Whistle</a>
- <a href="move-lookup.html?q=synthesis">Synthesis</a>
- <a href="move-lookup.html?q=play-rough">Play Rough</a>
- <a href="move-lookup.html?q=feint">Feint</a>
- <a href="move-lookup.html?q=charm">Charm</a>
- <a href="move-lookup.html?q=acupressure">Acupressure</a>

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
<div class="pokemon-tab-panel" id="pokemon-tabs-tsareena-panel-1">
Types: Grass • Egg Groups: Grass

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Leaf Guard
- Oblivious
- Sweet Veil *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=dazzling-gleam">TM54 - Dazzling Gleam</a>

**Held Item**
Grassy Seed

**Evolution Info**
Lv. 18

**Encounter Locations**
- Kipos Town — Grass (Day) (8%)
- Myrrini Island — Grass (Day) (20%)
- Tower of Dioxippus — Grass (Day) (10%)
- Tower of Dioxippus — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="steenee" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">52</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-low">48</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">48</span> |
| Speed | <span class="stat-value stat-mid">62</span> |
| Total | <span class="stat-value stat-low">295</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=double-slap">Double Slap</a> (Lv Evo)
- <a href="move-lookup.html?q=splash">Splash</a> (Lv 1)
- <a href="move-lookup.html?q=play-nice">Play Nice</a> (Lv 5)
- <a href="move-lookup.html?q=rapid-spin">Rapid Spin</a> (Lv 9)
- <a href="move-lookup.html?q=razor-leaf">Razor Leaf</a> (Lv 13)
- <a href="move-lookup.html?q=synthesis">Synthesis</a> (Lv 15)
- <a href="move-lookup.html?q=sweet-scent">Sweet Scent</a> (Lv 17)
- <a href="move-lookup.html?q=magical-leaf">Magical Leaf</a> (Lv 21)
- <a href="move-lookup.html?q=teeter-dance">Teeter Dance</a> (Lv 25)
- <a href="move-lookup.html?q=stomp">Stomp</a> (Lv 29)
- <a href="move-lookup.html?q=aromatic-mist">Aromatic Mist</a> (Lv 33)
- <a href="move-lookup.html?q=captivate">Captivate</a> (Lv 37)
- <a href="move-lookup.html?q=aromatherapy">Aromatherapy</a> (Lv 41)
- <a href="move-lookup.html?q=leaf-storm">Leaf Storm</a> (Lv 45)

**Egg Moves**
- <a href="move-lookup.html?q=grass-whistle">Grass Whistle</a>
- <a href="move-lookup.html?q=synthesis">Synthesis</a>
- <a href="move-lookup.html?q=play-rough">Play Rough</a>
- <a href="move-lookup.html?q=feint">Feint</a>
- <a href="move-lookup.html?q=charm">Charm</a>
- <a href="move-lookup.html?q=acupressure">Acupressure</a>

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
<div class="pokemon-tab-panel" id="pokemon-tabs-tsareena-panel-2">
Types: Grass • Egg Groups: Grass

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Leaf Guard
- Queenly Majesty
- Strong Legs *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=draining-kiss">TM15 - Draining Kiss</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=dazzling-gleam">TM54 - Dazzling Gleam</a>

**Held Item**
Grassy Seed

**Evolution Info**
Lv. knows Stomp
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="tsareena" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">72</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-high">98</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-high">98</span> |
| Speed | <span class="stat-value stat-high">97</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=trop-kick">Trop Kick</a> (Lv Evo)
- <a href="move-lookup.html?q=punishment">Punishment</a> (Lv 1)
- <a href="move-lookup.html?q=double-slap">Double Slap</a> (Lv 1)
- <a href="move-lookup.html?q=splash">Splash</a> (Lv 1)
- <a href="move-lookup.html?q=swagger">Swagger</a> (Lv 5)
- <a href="move-lookup.html?q=rapid-spin">Rapid Spin</a> (Lv 9)
- <a href="move-lookup.html?q=razor-leaf">Razor Leaf</a> (Lv 13)
- <a href="move-lookup.html?q=synthesis">Synthesis</a> (Lv 15)
- <a href="move-lookup.html?q=sweet-scent">Sweet Scent</a> (Lv 17)
- <a href="move-lookup.html?q=magical-leaf">Magical Leaf</a> (Lv 21)
- <a href="move-lookup.html?q=teeter-dance">Teeter Dance</a> (Lv 25)
- <a href="move-lookup.html?q=stomp">Stomp</a> (Lv 29)
- <a href="move-lookup.html?q=aromatic-mist">Aromatic Mist</a> (Lv 33)
- <a href="move-lookup.html?q=stomping-tantrum">Stomping Tantrum</a> (Lv 35)
- <a href="move-lookup.html?q=captivate">Captivate</a> (Lv 37)
- <a href="move-lookup.html?q=aromatherapy">Aromatherapy</a> (Lv 41)
- <a href="move-lookup.html?q=high-jump-kick">High Jump Kick</a> (Lv 43)
- <a href="move-lookup.html?q=leaf-storm">Leaf Storm</a> (Lv 45)
- <a href="move-lookup.html?q=thunderous-kick">Thunderous Kick</a> (Lv 49)
- <a href="move-lookup.html?q=power-whip">Power Whip</a> (Lv 53)

**Egg Moves**
- <a href="move-lookup.html?q=grass-whistle">Grass Whistle</a>
- <a href="move-lookup.html?q=synthesis">Synthesis</a>
- <a href="move-lookup.html?q=play-rough">Play Rough</a>
- <a href="move-lookup.html?q=feint">Feint</a>
- <a href="move-lookup.html?q=charm">Charm</a>
- <a href="move-lookup.html?q=acupressure">Acupressure</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mega-kick">Mega Kick</a>
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
#pokemon-tabs-tsareena-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-tsareena-panel-0 { display: block; }
#pokemon-tabs-tsareena-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-tsareena-panel-1 { display: block; }
#pokemon-tabs-tsareena-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-tsareena-panel-2 { display: block; }
</style>
</details>
