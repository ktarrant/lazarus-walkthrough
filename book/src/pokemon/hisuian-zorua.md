<details class="pokemon-card-container">
<summary>Hisuian Zorua (#198)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-hisuian-zorua">
<input type="radio" name="pokemon-tabs-hisuian-zorua-group" id="pokemon-tabs-hisuian-zorua-tab-0" checked>
<label for="pokemon-tabs-hisuian-zorua-tab-0">Hisuian Zorua</label>
<input type="radio" name="pokemon-tabs-hisuian-zorua-group" id="pokemon-tabs-hisuian-zorua-tab-1">
<label for="pokemon-tabs-hisuian-zorua-tab-1">Hisuian Zoroark</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-hisuian-zorua-panel-0">
Types: Normal / Ghost • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Illusion
- Fluffy *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)
- Ghost (0×)

*Weak to*
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=hex">TM23 - Hex</a>
- <a href="move-lookup.html?q=dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=sludge-bomb">TM36 - Sludge Bomb</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=will-o-wisp">TM51 - Will-O-Wisp</a>
- <a href="move-lookup.html?q=snarl">TM55 - Snarl</a>
- <a href="move-lookup.html?q=dark-pulse">TM59 - Dark Pulse</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-zorua" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">35</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-mid">85</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-mid">330</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 1)
- <a href="move-lookup.html?q=pursuit">Pursuit</a> (Lv 5)
- <a href="move-lookup.html?q=fake-tears">Fake Tears</a> (Lv 9)
- <a href="move-lookup.html?q=shadow-sneak">Shadow Sneak</a> (Lv 13)
- <a href="move-lookup.html?q=skitter-smack">Skitter Smack</a> (Lv 15)
- <a href="move-lookup.html?q=snarl">Snarl</a> (Lv 17)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 21)
- <a href="move-lookup.html?q=hex">Hex</a> (Lv 25)
- <a href="move-lookup.html?q=knock-off">Knock Off</a> (Lv 28)
- <a href="move-lookup.html?q=will-o-wisp">Will-O-Wisp</a> (Lv 32)
- <a href="move-lookup.html?q=shadow-ball">Shadow Ball</a> (Lv 35)
- <a href="move-lookup.html?q=shadow-claw">Shadow Claw</a> (Lv 37)
- <a href="move-lookup.html?q=uproar">Uproar</a> (Lv 40)
- <a href="move-lookup.html?q=embargo">Embargo</a> (Lv 42)
- <a href="move-lookup.html?q=infernal-parade">Infernal Parade</a> (Lv 45)
- <a href="move-lookup.html?q=nasty-plot">Nasty Plot</a> (Lv 49)
- <a href="move-lookup.html?q=extrasensory">Extrasensory</a> (Lv 53)
- <a href="move-lookup.html?q=night-daze">Night Daze</a> (Lv 57)

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-hisuian-zorua-panel-1">
Types: Normal / Ghost • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Illusion
- Fluffy *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.5×)
- Ghost (0×)

*Weak to*
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=hex">TM23 - Hex</a>
- <a href="move-lookup.html?q=dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=sludge-bomb">TM36 - Sludge Bomb</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=torment">TM41 - Torment</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=will-o-wisp">TM51 - Will-O-Wisp</a>
- <a href="move-lookup.html?q=snarl">TM55 - Snarl</a>
- <a href="move-lookup.html?q=dark-pulse">TM59 - Dark Pulse</a>
- <a href="move-lookup.html?q=rock-smash">HM06 - Rock Smash</a>

**Evolution Info**
Lv. 30
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="hisuian-zoroark" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">55</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-high">125</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-high">110</span> |
| Total | <span class="stat-value stat-mid">520</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=bitter-malice">Bitter Malice</a> (Lv Evo)
- <a href="move-lookup.html?q=shadow-sneak">Shadow Sneak</a> (Lv 1)
- <a href="move-lookup.html?q=scratch">Scratch</a> (Lv 1)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 1)
- <a href="move-lookup.html?q=pursuit">Pursuit</a> (Lv 5)
- <a href="move-lookup.html?q=fake-tears">Fake Tears</a> (Lv 9)
- <a href="move-lookup.html?q=shadow-sneak">Shadow Sneak</a> (Lv 13)
- <a href="move-lookup.html?q=skitter-smack">Skitter Smack</a> (Lv 15)
- <a href="move-lookup.html?q=snarl">Snarl</a> (Lv 17)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 21)
- <a href="move-lookup.html?q=hex">Hex</a> (Lv 25)
- <a href="move-lookup.html?q=knock-off">Knock Off</a> (Lv 28)
- <a href="move-lookup.html?q=will-o-wisp">Will-O-Wisp</a> (Lv 32)
- <a href="move-lookup.html?q=shadow-ball">Shadow Ball</a> (Lv 35)
- <a href="move-lookup.html?q=shadow-claw">Shadow Claw</a> (Lv 37)
- <a href="move-lookup.html?q=uproar">Uproar</a> (Lv 40)
- <a href="move-lookup.html?q=embargo">Embargo</a> (Lv 42)
- <a href="move-lookup.html?q=infernal-parade">Infernal Parade</a> (Lv 45)
- <a href="move-lookup.html?q=nasty-plot">Nasty Plot</a> (Lv 49)
- <a href="move-lookup.html?q=extrasensory">Extrasensory</a> (Lv 53)
- <a href="move-lookup.html?q=night-daze">Night Daze</a> (Lv 57)

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
#pokemon-tabs-hisuian-zorua-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-hisuian-zorua-panel-0 { display: block; }
#pokemon-tabs-hisuian-zorua-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-hisuian-zorua-panel-1 { display: block; }
</style>
</details>
