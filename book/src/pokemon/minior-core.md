<details class="pokemon-card-container">
<summary>Minior Core (#154)</summary>
Types: Rock / Flying • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shields Down

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.5×)
- Poison (0.5×)
- Ground (0×)
- Flying (0.5×)
- Bug (0.5×)

*Weak to*
- Water (2×)
- Electric (2×)
- Ice (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm04-calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm16-light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm26-earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=tm29-psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm33-reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=tm37-sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=tm39-rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm49-bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=tm54-dazzling-gleam">TM54 - Dazzling Gleam</a>

**Held Item**
Star Piece
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="minior-core" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-high">110</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-high">110</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-high">120</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a> (Lv 3)
- <a href="move-lookup.html?q=rollout">Rollout</a> (Lv 8)
- <a href="move-lookup.html?q=confuse-ray">Confuse Ray</a> (Lv 10)
- <a href="move-lookup.html?q=gust">Gust</a> (Lv 13)
- <a href="move-lookup.html?q=swift">Swift</a> (Lv 16)
- <a href="move-lookup.html?q=ancient-power">Ancient Power</a> (Lv 17)
- <a href="move-lookup.html?q=self-destruct">Self-Destruct</a> (Lv 22)
- <a href="move-lookup.html?q=stealth-rock">Stealth Rock</a> (Lv 24)
- <a href="move-lookup.html?q=acrobatics">Acrobatics</a> (Lv 27)
- <a href="move-lookup.html?q=take-down">Take Down</a> (Lv 29)
- <a href="move-lookup.html?q=autotomize">Autotomize</a> (Lv 31)
- <a href="move-lookup.html?q=cosmic-power">Cosmic Power</a> (Lv 34)
- <a href="move-lookup.html?q=power-gem">Power Gem</a> (Lv 37)
- <a href="move-lookup.html?q=dazzling-gleam">Dazzling Gleam</a> (Lv 40)
- <a href="move-lookup.html?q=double-edge">Double-Edge</a> (Lv 43)
- <a href="move-lookup.html?q=shell-smash">Shell Smash</a> (Lv 45)
- <a href="move-lookup.html?q=explosion">Explosion</a> (Lv 50)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=explosion">Explosion</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
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
