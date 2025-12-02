<details class="pokemon-card-container">
<summary>Arctovish (#411)</summary>
Types: Water / Ice • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Water Absorb
- Strong Jaw
- Snow Warning *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Ice (0.25×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Fighting (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm07-whirlpool">TM07 - Whirlpool</a>
- <a href="move-lookup.html?q=tm13-ice-beam">TM13 - Ice Beam</a>
- <a href="move-lookup.html?q=tm14-blizzard">TM14 - Blizzard</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm39-rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=hm03-surf">HM03 - Surf</a>
- <a href="move-lookup.html?q=hm07-waterfall">HM07 - Waterfall</a>
- <a href="move-lookup.html?q=hm08-dive">HM08 - Dive</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="arctovish" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">90</span> |
| Attack | <span class="stat-value stat-mid">90</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">55</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=powder-snow">Powder Snow</a> (Lv 1)
- <a href="move-lookup.html?q=water-gun">Water Gun</a> (Lv 1)
- <a href="move-lookup.html?q=protect">Protect</a> (Lv 7)
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a> (Lv 12)
- <a href="move-lookup.html?q=ancient-power">Ancient Power</a> (Lv 16)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 20)
- <a href="move-lookup.html?q=aurora-veil">Aurora Veil</a> (Lv 24)
- <a href="move-lookup.html?q=freeze-dry">Freeze-Dry</a> (Lv 28)
- <a href="move-lookup.html?q=ice-shard">Ice Shard</a> (Lv 33)
- <a href="move-lookup.html?q=super-fang">Super Fang</a> (Lv 35)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 38)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 40)
- <a href="move-lookup.html?q=aqua-jet">Aqua Jet</a> (Lv 44)
- <a href="move-lookup.html?q=wave-crash">Wave Crash</a> (Lv 48)
- <a href="move-lookup.html?q=fishious-rend">Fishious Rend</a> (Lv 52)
- <a href="move-lookup.html?q=icicle-crash">Icicle Crash</a> (Lv 55)
- <a href="move-lookup.html?q=blizzard">Blizzard</a> (Lv 58)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=icy-wind">Icy Wind</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
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
</details>
