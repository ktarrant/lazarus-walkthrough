<details class="pokemon-card-container">
<summary>Eternal Flower Floette (#313)</summary>
Types: Fairy • Egg Groups: -

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
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="eternal-flower-floette" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">74</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-mid">67</span> |
| Sp. Atk | <span class="stat-value stat-high">125</span> |
| Sp. Def | <span class="stat-value stat-high">128</span> |
| Speed | <span class="stat-value stat-high">92</span> |
| Total | <span class="stat-value stat-high">551</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=vine-whip">Vine Whip</a> (Lv 1)
- <a href="move-lookup.html?q=fairy-wind">Fairy Wind</a> (Lv 6)
- <a href="move-lookup.html?q=lucky-chant">Lucky Chant</a> (Lv 10)
- <a href="move-lookup.html?q=draining-kiss">Draining Kiss</a> (Lv 12)
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
- <a href="move-lookup.html?q=light-of-ruin">Light of Ruin</a> (Lv 50)
- <a href="move-lookup.html?q=petal-dance">Petal Dance</a> (Lv 51)
- <a href="move-lookup.html?q=solar-beam">Solar Beam</a> (Lv 58)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

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
</details>
